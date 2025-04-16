fn main() {
    println!("This is a test print to stdout");
    eprintln!("This is a test print to stderr");

    // Print the current directory
    let current_dir = std::env::current_dir().unwrap();
    println!("Current directory: {}", current_dir.display());

    // List files in the test_project directory
    let test_dir = current_dir.join("test_project");
    if test_dir.exists() {
        println!("test_project directory exists");
        for entry in std::fs::read_dir(test_dir).unwrap() {
            let entry = entry.unwrap();
            println!("  {}", entry.path().display());
        }
    } else {
        println!("test_project directory does not exist");
    }

    // Check if the canvas directory exists
    let canvas_dir = std::path::PathBuf::from("C:\\Users\\Tim\\Desktop\\LMS\\test_project\\canvas");
    if canvas_dir.exists() {
        println!("Canvas directory exists at: {}", canvas_dir.display());
        for entry in std::fs::read_dir(canvas_dir).unwrap() {
            let entry = entry.unwrap();
            println!("  {}", entry.path().display());

            // Check if it's the app directory
            if entry.path().file_name().unwrap() == "app" {
                let app_dir = entry.path();
                println!("    App directory exists at: {}", app_dir.display());
                for app_entry in std::fs::read_dir(app_dir).unwrap() {
                    let app_entry = app_entry.unwrap();
                    println!("    {}", app_entry.path().display());

                    // Check if it's the views directory
                    if app_entry.path().file_name().unwrap() == "views" {
                        let views_dir = app_entry.path();
                        println!("      Views directory exists at: {}", views_dir.display());
                        for views_entry in std::fs::read_dir(views_dir).unwrap() {
                            let views_entry = views_entry.unwrap();
                            println!("      {}", views_entry.path().display());

                            // Check if it's the courses directory
                            if views_entry.path().file_name().unwrap() == "courses" {
                                let courses_dir = views_entry.path();
                                println!("        Courses directory exists at: {}", courses_dir.display());
                                for courses_entry in std::fs::read_dir(courses_dir).unwrap() {
                                    let courses_entry = courses_entry.unwrap();
                                    println!("        {}", courses_entry.path().display());
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!("Canvas directory does not exist at: {}", canvas_dir.display());
    }

    // Check if the discourse directory exists
    let discourse_dir = std::path::PathBuf::from("C:\\Users\\Tim\\Desktop\\LMS\\test_project\\discourse");
    if discourse_dir.exists() {
        println!("Discourse directory exists at: {}", discourse_dir.display());
        for entry in std::fs::read_dir(discourse_dir).unwrap() {
            let entry = entry.unwrap();
            println!("  {}", entry.path().display());

            // Check if it's the app directory
            if entry.path().file_name().unwrap() == "app" {
                let app_dir = entry.path();
                println!("    App directory exists at: {}", app_dir.display());
                for app_entry in std::fs::read_dir(app_dir).unwrap() {
                    let app_entry = app_entry.unwrap();
                    println!("    {}", app_entry.path().display());

                    // Check if it's the views directory
                    if app_entry.path().file_name().unwrap() == "views" {
                        let views_dir = app_entry.path();
                        println!("      Views directory exists at: {}", views_dir.display());
                        for views_entry in std::fs::read_dir(views_dir).unwrap() {
                            let views_entry = views_entry.unwrap();
                            println!("      {}", views_entry.path().display());

                            // Check if it's the topics directory
                            if views_entry.path().file_name().unwrap() == "topics" {
                                let topics_dir = views_entry.path();
                                println!("        Topics directory exists at: {}", topics_dir.display());
                                for topics_entry in std::fs::read_dir(topics_dir).unwrap() {
                                    let topics_entry = topics_entry.unwrap();
                                    println!("        {}", topics_entry.path().display());
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!("Discourse directory does not exist at: {}", discourse_dir.display());
    }
}
