use std::collections::HashMap;


struct Solution {}

#[derive(Debug)]
struct Log {
  task_id: i32,
  start_time: i32,
  idle: i32,
}

impl Solution {
  fn parse(log: &String) -> (i32, bool, i32) {
    let split = log.split(":").collect::<Vec<&str>>();
    (split[0].parse::<i32>().unwrap(), split[1] == "start", split[2].parse::<i32>().unwrap())
  }

  pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
    let mut stack = Vec::<Log>::new();
    let mut time_map = vec![0; n as usize];

    for log in &logs {
      let (id, start, time) = Self::parse(&log);
      if start {
        stack.push(
          Log {task_id: id, start_time: time, idle : 0,}
        );
      } else {
        let l = stack.pop().unwrap();
        time_map[l.task_id as usize] += time - l.start_time - l.idle + 1;
        if let Some(log) = stack.last_mut() {
          log.idle += time - l.start_time + 1;
        }
      }
    }


    time_map
  }
}

fn main() {

}
