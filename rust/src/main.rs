

#[derive(PartialEq)]
pub enum Color {
  BLUE,
  RED,
  YELLOW,
  GREEN,
}

fn main() {
  println!("Hello, world!");
  evaluate(vec!(), vec!());
}


fn evaluate(secret:Vec<Color>, proposal:Vec<Color>) -> Vec<i32> {
  let mut ok = 0;
  let mut ko = 0;
  let mut checked:Vec<bool> = vec![false; secret.len()];

  for (proposal_index, proposal_color) in proposal.iter().enumerate() {
    let secret_color = &secret[proposal_index];
    if proposal_color == secret_color {
      ok = ok + 1;
      checked[proposal_index] = true;
    }
  }

  for proposal_color in proposal.iter() {
    for (secret_index, secret_color) in secret.iter().enumerate() {
      if !checked[secret_index] && secret_color == proposal_color {
        ko = ko + 1;
        checked[secret_index] = true;
        break;
      }
    }
  }

  vec!(ok, ko)
}

#[cfg(test)]
mod tests {
    #[test]
    fn evaluate_b_b_1_0() {
      use Color;
      use evaluate;

      assert_eq!(evaluate(vec!(Color::BLUE), vec!(Color::BLUE)), vec!(1,0));
    }

    #[test]
    fn evaluate_r_b_0_0() {
      use Color;
      use evaluate;

      assert_eq!(evaluate(vec!(Color::RED), vec!(Color::BLUE)), vec!(0,0));
    }

    #[test]
    fn evaluate_br_br_2_0() {
      use Color;
      use evaluate;

      assert_eq!(evaluate(vec!(Color::BLUE, Color::RED), vec!(Color::BLUE, Color::RED)), vec!(2,0));
    }

    #[test]
    fn evaluate_rb_br_0_2() {
      use Color;
      use evaluate;

      assert_eq!(evaluate(vec!(Color::RED, Color::BLUE), vec!(Color::BLUE, Color::RED)), vec!(0,2));
    }

    #[test]
    fn evaluate_rrb_brb_2_1() {
      use Color;
      use evaluate;

      assert_eq!(evaluate(vec!(Color::RED, Color::RED, Color::BLUE), vec!(Color::BLUE, Color::RED, Color::BLUE)), vec!(2,1));
    }

    #[test]
    fn evaluate_rrb_bbb_1_0() {
      use Color;
      use evaluate;

      assert_eq!(evaluate(vec!(Color::RED, Color::RED, Color::BLUE), vec!(Color::BLUE, Color::BLUE, Color::BLUE)), vec!(1,0));
    }

    #[test]
    fn evaluate_rrb_bbr_2_0() {
      use Color;
      use evaluate;

      assert_eq!(evaluate(vec!(Color::RED, Color::RED, Color::BLUE), vec!(Color::BLUE, Color::BLUE, Color::RED)), vec!(0,2));
    }

    #[test]
    fn evaluate_bryg_byrb_1_2() {
      use Color;
      use evaluate;

      assert_eq!(evaluate(vec!(Color::BLUE, Color::RED, Color::YELLOW, Color::GREEN), vec!(Color::BLUE, Color::YELLOW, Color::RED, Color::BLUE)), vec!(1,2));
    }

    #[test]
    fn evaluate_bryy_yyrb_0_4() {
      use Color;
      use evaluate;

      assert_eq!(evaluate(vec!(Color::BLUE, Color::RED, Color::YELLOW, Color::YELLOW), vec!(Color::YELLOW, Color::YELLOW, Color::RED, Color::BLUE)), vec!(0,4));
    }
}