

#[derive(Debug, Clone)]
pub enum ParseType {
    /// latex, .tex file
    Latex,
    /// written directily into a pdf, default mode
    Raw,
}
