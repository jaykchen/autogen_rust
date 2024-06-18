use core::task;

use anyhow::Result;
use autogen_rust::{immutable_agent::*, task_ledger};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let user_proxy = ImmutableAgent::simple("user_proxy", "");

    // let (mut task_ledger, solution) = user_proxy
    //     // .planning("tell me a joke")
    //     .planning("Today is 2024-03-18. Write a blogpost about the stock price performance of Nvidia in the past month")
    //     .await;

    // if task_ledger.task_list.is_empty() && solution.is_some() {
    //     println!("solution: {:?} ", solution);
    //     std::process::exit(0);
    // }

    // loop {
    //     let task = task_ledger
    //         .current_task()
    //         .unwrap_or("no task found".to_string());

        let _ = user_proxy
            .code_with_python("Write python code to show the stock price performance of Nvidia in the past month")
            .await;
    //     let res_alt = user_proxy
    //         .iterate_next_step(&res, &task)
    //         .await
    //         .unwrap_or("no result generated".to_string());
    //     println!("{:?}", res_alt.clone());

    //     tokio::time::sleep(std::time::Duration::from_secs(60)).await;

    //     if !task_ledger.record_solution(res_alt) {
    //         break;
    //     }
    // }

    // println!(
    //     "{:?}",
    //     &task_ledger
    //         .solution_list
    //         .last()
    //         .unwrap_or(&"no final result".to_string())
    // );

    Ok(())
}
