// Styles
pub const CAROUSEL: &str = r#"
    width: 100%;
    overflow: hidden;
    position: relative;
    height: 100%;
"#;

pub const CAROUSEL_INNER: &str = r#"
    display: flex;
    transition: transform 2s;
    height: 100%;
"#;

pub const CAROUSEL_ITEM: &str = r#"
    flex: 0 0 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    "#;

pub const CAROUSEL_HEADER: &str = r#"
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    margin-bottom: 1em;
"#;
    
pub const CAROUSEL_CONTENT: &str = r#"
    height: 100%;
    width: 90%;
    background-color: #666699;
    color: #ccf;
    display: flex;
    flex-direction: column;
    align-items: center;
    overflow-y: scroll;
    overflow-x: hidden;
"#;

pub const PROJECT_DESCRIPTION: &str = r#"
    max-width: 900px;
    padding: 0 0.5em;
    text-align: center;
"#;

pub const PROJECT_TITLE: &str = r#"
    margin-bottom: 0;
    flex-grow: 1;
    text-align: center;
"#;

pub const PROJECT_IMG: &str = r#"
    max-width: 90%;
    max-height: 600px;
"#;

pub const BTN_LEFT: &str = r#"
    left: 0;
"#;

pub const BTN_RIGHT: &str = r#"
    right: 0;
"#;

pub const BTN_ARROW: &str = r#"
    width: 50%;
"#;

pub const GITHUB: &str = r#"
    margin: 0.5em;
    cursor: pointer;
"#;

pub const LIVE: &str = r#"
    margin: 0.5em;
    cursor: pointer;
"#;

pub const LOGO_CONTAINER: &str = r#"
    display: flex;
    justify-content: center;
    background-color: #334;
    padding: 0 2em;
    border-radius: 2em;
    margin-bottom: 2em;
"#;

pub const LOGO: &str = r#"
    height: 50px;
    margin: 1em;
"#;
