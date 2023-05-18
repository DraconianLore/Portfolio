// Styles
pub const CAROUSEL: &str = r#"
        width: 100%;
        overflow: hidden;
        position: relative;
        height: 100%;
    "#;

pub const CAROUSEL_INNER: &str = r#"
    display: flex;
    animation: carousel-anim 90s infinite;
    height: 100%;

"#;

pub const CAROUSEL_ITEM: &str = r#"
    flex: 0 0 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    "#;
    
pub const CAROUSEL_CONTENT: &str = r#"
    height: 100%;
    width: 90%;
    background-color: #666699;
    color: #ccf;
    display: flex;
    flex-direction: column;
    align-items: center;
"#;
