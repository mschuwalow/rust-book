use department::runner::Runner;
use department::term::*;

fn main() {
    let mut runner = Runner::new();
    println!("Listening for commands:\n");
    loop {
        match read_command() {
            Ok(cmd) => match cmd {
                Command::AddUser { user, department } => {
                    runner.add_user(&department, &user);
                    println!("==> Added user {} to department {}.", user, department);
                }
                Command::ListAllInDepartment { department } => {
                    let users = runner.list_all_users_in_dep(&department);
                    println!("==> Users in department {}: {:?}", department, users);
                }
                Command::ListAll() => {
                    let users = runner.list_all_users();
                    println!("==> All users: {:?}", users);
                }
            },
            Err(err) => println!("Invalid input: {}", err),
        }
    }
}
