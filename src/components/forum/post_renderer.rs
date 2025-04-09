use leptos::*;
use pulldown_cmark::{html, Parser, Options};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use std::sync::OnceLock;
use web_sys::{DomParser, SupportedType};
use regex::Regex;
use std::collections::HashMap;

// Lazy initialized syntax highlighting components
static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

fn get_syntax_set() -> &'static SyntaxSet {
    SYNTAX_SET.get_or_init(|| SyntaxSet::load_defaults_newlines())
}

fn get_theme_set() -> &'static ThemeSet {
    THEME_SET.get_or_init(|| ThemeSet::load_defaults())
}

// Optimize markdown rendering with memoization caching
#[derive(Clone)]
struct RenderedMarkdown {
    html: String,
    last_updated: chrono::DateTime<chrono::Utc>,
}

type MarkdownCache = moka::future::Cache<String, RenderedMarkdown>;

// Create singleton cache
fn get_markdown_cache() -> &'static MarkdownCache {
    static CACHE: OnceLock<MarkdownCache> = OnceLock::new();
    CACHE.get_or_init(|| {
        moka::future::Cache::builder()
            .max_capacity(100)
            .time_to_live(std::time::Duration::from_secs(300))
            .build()
    })
}

// Optimized post renderer component with advanced features
#[component]
pub fn PostRenderer(
    content: Signal<String>,
    #[prop(optional)] highlight_syntax: bool,
    #[prop(optional)] render_math: bool,
    #[prop(optional)] sanitize: bool,
) -> impl IntoView {
    let theme = use_context::<Signal<String>>().unwrap_or_else(|| create_signal("light".to_string()).0);
    
    // Create memoized rendered content
    let rendered_content = create_memo(move |_| {
        let markdown = content.get();
        let current_theme = theme.get();
        let cache_key = format!("{}:{}:{}:{}:{}", 
            markdown, current_theme, highlight_syntax, render_math, sanitize);
        
        // Try to get from cache
        let cache = get_markdown_cache();
        if let Some(cached) = cache.get(&cache_key) {
            return cached.html;
        }
        
        // Render markdown with advanced features
        let rendered = render_markdown(
            &markdown, 
            &current_theme,
            highlight_syntax,
            render_math,
            sanitize
        );
        
        // Cache the result
        cache.insert(
            cache_key, 
            RenderedMarkdown {
                html: rendered.clone(),
                last_updated: chrono::Utc::now()
            }
        );
        
        rendered
    });
    
    // Render using dangerouslySetInnerHTML for the rendered markdown
    view! {
        <div 
            class="post-content"
            dangerously_set_inner_html={move || rendered_content.get()}
        ></div>
    }
}

// Actual markdown rendering with all features
fn render_markdown(
    markdown: &str, 
    theme_name: &str,
    highlight_syntax: bool,
    render_math: bool,
    sanitize: bool
) -> String {
    // Set up markdown parser with common extensions
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    
    let parser = Parser::new_ext(markdown, options);
    
    // Initial markdown to HTML conversion
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    // Post-process for syntax highlighting if enabled
    if highlight_syntax {
        html_output = process_code_blocks(&html_output, theme_name);
    }
    
    // Process LaTeX if enabled
    if render_math {
        html_output = process_math(&html_output);
    }
    
    // Sanitize if requested
    if sanitize {
        html_output = sanitize_html(&html_output);
    }
    
    html_output
}

// Process code blocks with syntax highlighting
fn process_code_blocks(html: &str, theme_name: &str) -> String {
    // Regular expression to find code blocks with language info
    let code_block_regex = Regex::new(r"<pre><code class=\"language-([^\"]+)\">(.*?)</code></pre>").unwrap();
    
    let syntax_set = get_syntax_set();
    let theme_set = get_theme_set();
    
    // Select theme based on user preference
    let theme = match theme_name {
        "dark" => &theme_set.themes["Solarized (dark)"],
        _ => &theme_set.themes["Solarized (light)"],
    };
    
    code_block_regex.replace_all(html, |caps: &regex::Captures| {
        let lang = &caps[1];
        let code = html_escape::decode_html_entities(&caps[2]);
        
        // Find syntax or default to Plain Text
        let syntax = syntax_set
            .find_syntax_by_token(lang)
            .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
        
        // Highlight the code
        match highlighted_html_for_string(&code, syntax_set, syntax, theme) {
            Ok(highlighted) => {
                format!("<pre class=\"syntax-highlighted language-{}\"><code>{}</code></pre>", lang, highlighted)
            },
            Err(_) => {
                // Fall back to non-highlighted code
                format!("<pre><code class=\"language-{}\">{}</code></pre>", lang, code)
            }
        }
    }).to_string()
}

// Process math expressions (LaTeX)
fn process_math(html: &str) -> String {
    // Process inline math: $formula$
    let inline_regex = Regex::new(r"\$([^\$\n]+)\$").unwrap();
    
    let mut result = inline_regex.replace_all(html, |caps: &regex::Captures| {
        format!("<span class=\"math-inline\" data-latex=\"{}\"></span>", &caps[1])
    }).to_string();
    
    // Process block math: $$formula$$
    let block_regex = Regex::new(r"\$\$([\s\S]+?)\$\$").unwrap();
    
    result = block_regex.replace_all(&result, |caps: &regex::Captures| {
        format!("<div class=\"math-block\" data-latex=\"{}\"></div>", &caps[1])
    }).to_string();
    
    result
}

// Simple HTML sanitizer
fn sanitize_html(html: &str) -> String {
    // This is a basic implementation - consider using a proper HTML sanitizer crate
    // like ammonia for production code
    let allowed_tags = vec![
        "p", "br", "h1", "h2", "h3", "h4", "h5", "h6",
        "blockquote", "pre", "code", "ul", "ol", "li",
        "strong", "em", "i", "b", "a", "img", "table",
        "thead", "tbody", "tr", "th", "td", "hr", "div", "span",
    ];
    
    let allowed_attrs = HashMap::from([
        ("a", vec!["href", "title", "target", "rel"]),
        ("img", vec!["src", "alt", "title", "width", "height"]),
        ("div", vec!["class", "data-latex"]),
        ("span", vec!["class", "data-latex"]),
        ("code", vec!["class"]),
        ("pre", vec!["class"]),
    ]);
    
    // Parse HTML
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("no document on window");
    let parser = document
        .create_element("template")
        .expect("could not create template element");
    
    parser.set_inner_html(html);
    let fragment = parser.content();
    
    // Process nodes
    sanitize_node(&document, &fragment, &allowed_tags, &allowed_attrs);
    
    // Return sanitized HTML
    parser.inner_html()
}

fn sanitize_node(
    document: &web_sys::Document,
    node: &web_sys::Node,
    allowed_tags: &[&str],
    allowed_attrs: &HashMap<&str, Vec<&str>>,
) {
    let mut i = 0;
    while let Some(child) = node.child_nodes().item(i) {
        let node_type = child.node_type();
        
        if node_type == 1 {
            // Element node
            if let Some(element) = child.dyn_ref::<web_sys::Element>() {
                let tag_name = element.tag_name().to_lowercase();
                
                if !allowed_tags.contains(&tag_name.as_str()) {
                    // Remove disallowed tag
                    node.remove_child(&child).unwrap();
                    continue;
                }
                
                // Process attributes
                if let Some(allowed_attributes) = allowed_attrs.get(tag_name.as_str()) {
                    let attributes = element.get_attribute_names();
                    
                    for j in 0..attributes.length() {
                        if let Some(attr_name) = attributes.item(j) {
                            if !allowed_attributes.contains(&attr_name.as_str()) {
                                element.remove_attribute(&attr_name).unwrap();
                            }
                        }
                    }
                }
                
                // Recursively sanitize children
                sanitize_node(document, &child, allowed_tags, allowed_attrs);
            }
        } else if node_type != 3 {
            // Not a text node, remove it
            node.remove_child(&child).unwrap();
            continue;
        }
        
        i += 1;
    }
}