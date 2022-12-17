
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Spot {
  Wall,
  Empty
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Pos {
  x: i32,
  y: i32
}

impl Pos {
  fn new(x: i32, y: i32) -> Self {
    Pos { x, y }
  }

  fn next(&self, m: &Maze) -> Vec<Self> {
    assert!(self.x < m.size && self.y < m.size);

    let positions = vec![
      Pos::new(self.x - 1, self.y),
      Pos::new(self.x + 1, self.y),
      Pos::new(self.x, self.y - 1),
      Pos::new(self.x, self.y + 1)
    ];

    positions
      .into_iter()
      .filter(|ele| {
        ele.x < m.size && ele.y < m.size &&
        ele.x >= 0 && ele.y >= 0 && m.open(&ele)
      })
      .collect()
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Maze {
  size: i32,
  target: Pos,
  cells: Vec<Spot>
}

impl Maze {
  pub fn new(size: i32) -> Self {
    let s = size as usize * size as usize;
    Maze { size, target: Pos::new(size-1, size-1), cells: vec![Spot::Empty; s] }
  }

  pub fn solve(&self, start: Pos) -> Option<Vec<Pos>> {
    
    fn fn_for_pos(m: &Maze, p: Pos, mut p_wl: Vec<Pos>, path: Vec<Pos>, mut path_wl: Vec<Vec<Pos>>, rsf: Vec<Pos>) -> Option<Vec<Pos>> {
      if p == m.target {
        // find the shorter of the current path to target and the current shortest path to target
        let mut n_rsf = path;
        n_rsf.push(p);
        n_rsf = if rsf.len() < n_rsf.len() && rsf.len() > 0 { rsf } else { n_rsf };
        
        // only relevant in debug mode
        stacker::maybe_grow(RED_ZONE, GROW, || {
          // if the target is reached, recurse down worklists and find next path to target
          fn_for_lop(m, p_wl, path_wl, n_rsf)
        })  
      } else if path.contains(&p) {
        // only relevant in debug mode
        stacker::maybe_grow(RED_ZONE, GROW, || {
          // skip this current position; its already been explored (in its own path)
          fn_for_lop(m, p_wl, path_wl, rsf)
        })
      } else {
        let next_ps = p.next(m);
        let next_ps_len = next_ps.len();
        
        let mut n_p_wl = next_ps;
        n_p_wl.append(&mut p_wl);

        let mut n_path = path;
        n_path.push(p);
        let mut n_path_wl = vec![];
        for _ in 0..next_ps_len {
          n_path_wl.push(n_path.clone());
        }
        n_path_wl.append(&mut path_wl);
        
        // only relevant in debug mode
        stacker::maybe_grow(RED_ZONE, GROW, || {
          // recurse down next positions in worklists until target is reached (check first part of if statement for what happens after)
          fn_for_lop(m, n_p_wl, n_path_wl, rsf)
        })
      }
    }

    fn fn_for_lop(m: &Maze, p_wl: Vec<Pos>, path_wl: Vec<Vec<Pos>>, rsf: Vec<Pos>) -> Option<Vec<Pos>> {
      // check return condition -> all positions have been explored, return shortest path or none if unsolveable
      if p_wl.len() == 0 {
        if rsf.len() == 0 {
          None
        } else {
          Some(rsf)
        }
      } else {
        let p_head = p_wl[0];
        let mut p_tail = vec![];
        
        if p_wl.len() > 0 {
          p_tail = p_wl[1..].to_owned();
        }

        let mut path_head = vec![];
        let mut path_tail = vec![];
        
        if path_wl.len() > 0 {
          path_head = path_wl[0].clone();
          path_tail = path_wl[1..].to_owned();
        }
        
        // only relevant in debug mode
        stacker::maybe_grow(RED_ZONE, GROW, || {
          // continue to work through worklists
          fn_for_pos(m, p_head, p_tail.to_vec(), path_head, path_tail.to_vec(), rsf)
        })
      }
    }
  
    // only relevant in debug mode
    stacker::maybe_grow(RED_ZONE, GROW, || {
      // call position function on initial position and worklists
      fn_for_pos(self, start, vec![], vec![], vec![], vec![])
    })
  }

  fn render(&self, path: Vec<Pos>) {
    let mut s = String::new();

    self.cells
      .iter()
      .enumerate()
      .for_each(|(i, _)| {
        if i % self.size as usize == 0 { s.push('\n'); }
        // identify x and y values relative to width
        let x: i32 = i as i32 % self.size;
        let y: i32 = i as i32 / self.size;
        
        // decide the next character to print
        let next_char = if path.contains(&Pos::new(x, y)) {
          PATH_CHAR
        } else if self.get(x, y) == Spot::Empty {
          EMPTY_CHAR
        } else {
          WALL_CHAR
        };
        
        if next_char == PATH_CHAR {
          // Bold red ansi escape sequence and stop sequence
          let start = "\x1b[1;31m";
          let stop  = "\x1b[0m";
          s.push_str(start);
          s.push(next_char);
          s.push_str(stop);
        } else {
          s.push(next_char);
        }
        
        s.push(' ');
      });

    use std::io::Write;

    let mut stdout = std::io::BufWriter::new(std::io::stdout());
    stdout.write_all(s.as_bytes()).unwrap(); // expect no failure
    
    print!("\n");
  }

  fn get(&self, x: i32, y: i32) -> Spot {
    assert!(x < self.size);
    assert!(y < self.size);
    
    self.cells[y as usize * self.size as usize + x as usize]
  }

  fn open(&self, p: &Pos) -> bool {
    self.get(p.x, p.y) == Spot::Empty
  }
}

const RED_ZONE: usize = 8 * 1024; // 32Kb
const GROW: usize     = 32 * 1024; // 1Mb

const PATH_CHAR: char  = '+';
const WALL_CHAR: char  = '@';
const EMPTY_CHAR: char = '.';

fn main() {
  let W = Spot::Wall;
  let E = Spot::Empty;
  
  let mut m = Maze::new(10);
  
  m.cells = vec![
    E, E, E, E, E, E, E, E, E, E,
    W, W, E, W, W, W, W, W, W, E,
    E, E, E, W, W, W, W, E, E, E,
    E, W, E, E, W, W, W, E, W, W,
    E, W, W, E, E, E, W, E, E, E,
    E, W, W, E, W, E, W, W, W, E,
    E, W, W, E, W, E, W, E, E, E,
    E, W, W, E, E, E, W, E, W, W,
    E, E, E, E, W, E, W, E, E, E,
    W, W, W, W, W, E, E, E, E, E,
  ];

  if let Some(path) = m.solve(Pos::new(0, 0)) {
    println!("Maze solved!");
    m.render(path);
  } else {
    println!("Maze is unsolveable!");
    m.render(vec![]);
  }

  println!();
}


