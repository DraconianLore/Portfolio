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
    margin: 0.5em 0.5em 1em;
"#;
    
pub const CAROUSEL_CONTENT: &str = r#"
    height: 100%;
    width: 90vw;
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
    font-size: 1.5em;
"#;

pub const IMG_CONTAINER: &str = r#"
    width: 100%;
    display: flex;
    justify-content: center;
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
    margin-left: 0.5em;
    cursor: pointer;
"#;

pub const LIVE: &str = r#"
    margin-right: 0.5em;
    cursor: pointer;
"#;

pub const TECH_STACK: &str = r#"
    padding: 1em 1em 0;
"#;

pub const LOGO_CONTAINER: &str = r#"
    display: flex;
    justify-content: center;
    background-color: #334;
    border-radius: 2em;
    margin-bottom: 2em;
    flex-wrap: wrap;
    max-width: 90%;
"#;

pub const LOGO: &str = r#"
    max-height: 50px;
    margin: 1em;
"#;
