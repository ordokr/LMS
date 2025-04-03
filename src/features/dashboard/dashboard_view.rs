use leptos::*;

#[component]
pub fn Dashboard(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="dashboard">
            <h1 class="page-title">"Dashboard"</h1>
            
            <section class="dashboard-section upcoming-events">
                <h2>"Upcoming Events"</h2>
                <div class="card-grid">
                    <div class="card">
                        <h3>"Assignment Due: Project Proposal"</h3>
                        <p>"Course: Introduction to Web Development"</p>
                        <p class="due-date">"Due: April 5, 2025"</p>
                    </div>
                    <div class="card">
                        <h3>"Quiz: Programming Fundamentals"</h3>
                        <p>"Course: Intro to Computer Science"</p>
                        <p class="due-date">"Due: April 7, 2025"</p>
                    </div>
                </div>
            </section>
            
            <section class="dashboard-section courses">
                <h2>"My Courses"</h2>
                <div class="card-grid">
                    <div class="card course-card">
                        <h3>"CS 101: Introduction to Computer Science"</h3>
                        <p>"Learn the fundamentals of computer science and programming."</p>
                        <a href="/courses/1" class="button">"Go to Course"</a>
                    </div>
                    <div class="card course-card">
                        <h3>"WEB 200: Web Development"</h3>
                        <p>"Frontend and backend web development techniques."</p>
                        <a href="/courses/2" class="button">"Go to Course"</a>
                    </div>
                    <div class="card course-card">
                        <h3>"MATH 150: Calculus I"</h3>
                        <p>"Introduction to limits, derivatives, and integrals."</p>
                        <a href="/courses/3" class="button">"Go to Course"</a>
                    </div>
                </div>
            </section>
            
            <section class="dashboard-section recent-activity">
                <h2>"Recent Activity"</h2>
                <ul class="activity-list">
                    <li class="activity-item">
                        <span class="activity-icon forum-post">"💬"</span>
                        <div class="activity-content">
                            <p>"New discussion post in 'Introduction to Computer Science'"</p>
                            <p class="activity-meta">"2 hours ago"</p>
                        </div>
                    </li>
                    <li class="activity-item">
                        <span class="activity-icon assignment-submit">"📝"</span>
                        <div class="activity-content">
                            <p>"You submitted 'Assignment 2' in 'Web Development'"</p>
                            <p class="activity-meta">"Yesterday"</p>
                        </div>
                    </li>
                    <li class="activity-item">
                        <span class="activity-icon grade-posted">"🎯"</span>
                        <div class="activity-content">
                            <p>"Grade posted for 'Quiz 1' in 'Calculus I'"</p>
                            <p class="activity-meta">"2 days ago"</p>
                        </div>
                    </li>
                </ul>
            </section>
        </div>
    }
}