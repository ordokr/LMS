use crate::models::assignment::Assignment;

#[tokio::test]
async fn test_assignment() {
    // Example test logic for assignment
    let assignment = Assignment {
        id: 1,
        name: "Test Assignment".to_string(),
        description: "This is a test assignment.".to_string(),
    };
    assert_eq!(assignment.name, "Test Assignment");
}
