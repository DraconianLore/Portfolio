// Structure of Project
pub struct Project<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub tech_stack: &'a str,
    pub image: &'a str,
}

// Import project files here
pub mod project1;
pub mod project2;
pub mod project3;
pub mod project4;
pub mod project5;


// Add new projects here in desired slide order
pub fn GET_PROJECT(i: i32) -> &'static Project<'static> {
    let current_project = match i{
        0=>&project1::DETAILS,
        1=>&project2::DETAILS,
        2=>&project3::DETAILS,
        3=>&project4::DETAILS,
        4=>&project5::DETAILS,
        _=>&project1::DETAILS,
    };
    return current_project
}
