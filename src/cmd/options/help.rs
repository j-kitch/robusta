use crate::cmd::{Control, Robusta};

const HELP_MESSAGE: &str = "Usage: robusta [-options] class [args...]
           (to execute a class)
where options include:
    -d32	  use a 32-bit data model if available
    -d64	  use a 64-bit data model if available
    -cp <class search path of directories and zip/jar files>
    -classpath <class search path of directories and zip/jar files>
                  A : separated list of directories, JAR archives,
                  and ZIP archives to search for class files.
    -version      print product version and exit
                  require the specified version to run
    -showversion  print product version and continue
    -? -help      print this help message
";

pub fn help(_: &mut Robusta, _: &[String], idx: usize) -> (Control, usize) {
    println!("{}", HELP_MESSAGE);
    (Control::Exit, idx + 1)
}
