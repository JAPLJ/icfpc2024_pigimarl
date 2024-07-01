use std::{
    io,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pt {
    x: i64,
    y: i64,
}

impl Pt {
    fn new(x: i64, y: i64) -> Pt {
        Pt { x, y }
    }

    fn norm2(&self) -> i64 {
        self.x * self.x + self.y * self.y
    }

    fn abs(&self) -> f64 {
        (self.norm2() as f64).sqrt()
    }

    fn arg(&self) -> f64 {
        (self.y as f64).atan2(self.x as f64)
    }

    fn acc_as_tenkey(&self) -> usize {
        match self {
            Pt { x: -1, y: -1 } => 1,
            Pt { x: 0, y: -1 } => 2,
            Pt { x: 1, y: -1 } => 3,
            Pt { x: -1, y: 0 } => 4,
            Pt { x: 0, y: 0 } => 5,
            Pt { x: 1, y: 0 } => 6,
            Pt { x: -1, y: 1 } => 7,
            Pt { x: 0, y: 1 } => 8,
            Pt { x: 1, y: 1 } => 9,
            _ => panic!("invalid acc_as_tenkey"),
        }
    }
}

impl Add<Pt> for Pt {
    type Output = Pt;

    fn add(self, other: Pt) -> Pt {
        Pt {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Pt> for Pt {
    type Output = Pt;

    fn sub(self, other: Pt) -> Pt {
        Pt {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i64> for Pt {
    type Output = Pt;

    fn mul(self, other: i64) -> Pt {
        Pt {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<i64> for Pt {
    type Output = Pt;

    fn div(self, other: i64) -> Pt {
        Pt {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct MovingPt {
    p: Pt,
    v: Pt,
}

impl MovingPt {
    fn new(p: Pt, v: Pt) -> MovingPt {
        MovingPt { p, v }
    }
}

fn read_input() -> Vec<Pt> {
    let lines = io::stdin().lines();
    let mut pts = Vec::new();
    for line in lines {
        let line = line.unwrap();
        if let Some((x, y)) = line.trim().split_once(" ") {
            let x = x.trim().parse().unwrap();
            let y = y.trim().parse().unwrap();
            pts.push(Pt::new(x, y));
        }
    }
    pts
}

struct Env {
    pts: Vec<Pt>,
}

impl Env {
    fn new(pts: Vec<Pt>) -> Env {
        Env { pts }
    }

    fn calc_a(&self, target: usize, _vis: &[bool], mp: MovingPt) -> Pt {
        let mut min_a = Pt::new(0, 0);

        'outer: for total_t in 1.. {
            let mut d = self.pts[target] - mp.p;
            d = d - mp.v * total_t;
            let mut first_a = None;
            let mut vv = mp.v;
            for t in 0..total_t {
                let rt = total_t - t;
                let mut a = Pt::new((d.x / rt).signum(), (d.y / rt).signum());
                if (vv.x + a.x).abs() > 50 {
                    a.x = 0;
                }
                if (vv.y + a.y).abs() > 50 {
                    a.y = 0;
                }
                vv = vv + a;
                if t == 0 {
                    first_a = Some(a);
                }
                d = d - a * rt;
                if d == Pt::new(0, 0) {
                    min_a = first_a.unwrap();
                    break 'outer;
                }
            }
        }

        min_a
    }

    fn order_stripe(&self, div: usize) -> Vec<usize> {
        let n = self.pts.len();
        let mut max_dist = 0.0;
        for i in 0..n {
            let dist = self.pts[i].abs();
            if dist > max_dist {
                max_dist = dist;
            }
        }

        let mut stripe = vec![vec![]; div];
        for i in 0..n {
            let dist = self.pts[i].abs();
            let idx = ((dist / max_dist * div as f64) as usize).min(div - 1);
            stripe[idx].push(i);
        }

        let mut ord = vec![];
        for s in 0..div {
            stripe[s].sort_by(|&i, &j| self.pts[i].arg().partial_cmp(&self.pts[j].arg()).unwrap());
            ord.extend(stripe[s].clone());
        }

        ord
    }

    fn order_sq_dist(&self, div: usize) -> Vec<Vec<usize>> {
        let n = self.pts.len();
        let mut max_dist = 0;
        for i in 0..n {
            let dist = self.pts[i].x.abs().max(self.pts[i].y.abs());
            if dist > max_dist {
                max_dist = dist;
            }
        }

        let mut stripe = vec![vec![]; div];
        for i in 0..n {
            let dist = self.pts[i].x.abs().max(self.pts[i].y.abs());
            let idx = ((dist * (div as i64) / max_dist) as usize).min(div - 1);
            stripe[idx].push(i);
        }

        let mut ord = vec![];
        for s in 0..div {
            stripe[s].sort_by(|&i, &j| self.pts[i].arg().partial_cmp(&self.pts[j].arg()).unwrap());
            ord.push(stripe[s].clone());
        }

        ord
    }

    fn order(&self) -> Vec<usize> {
        let mut cur = Pt::new(0, 0);
        let mut res = Vec::new();
        let mut vis = vec![false; self.pts.len()];
        let n = self.pts.len();

        for _ in 0..n {
            let mut min_dist = i64::MAX;
            let mut min_i = 0;
            for i in 0..n {
                if vis[i] {
                    continue;
                }
                let dist = (self.pts[i] - cur).norm2();
                if dist < min_dist {
                    min_dist = dist;
                    min_i = i;
                }
            }
            res.push(min_i);
            vis[min_i] = true;
            cur = self.pts[min_i];
        }

        let mut loops = 0;
        loop {
            loops += 1;
            // eprintln!("loop {}", loops);
            if loops >= n {
                break;
            }
            let mut furthest = 0;
            let mut max_dist = 0;
            for i in 0..n - 1 {
                let dist = (self.pts[res[i]] - self.pts[res[i + 1]]).norm2();
                if dist > max_dist {
                    max_dist = dist;
                    furthest = i + 1;
                }
            }

            fn insert_cost(to0: Pt, to1: Pt, fr0: Pt, p: Pt, fr1: Option<Pt>) -> f64 {
                let mut diff = 0.0;
                diff += (to0 - p).abs() + (to1 - p).abs() - (to0 - to1).abs();
                diff -= (fr0 - p).abs();
                if let Some(fr1) = fr1 {
                    diff -= (fr1 - p).abs();
                    diff += (fr0 - fr1).abs();
                }
                diff
            }

            let insert_at = |pos: usize| {
                insert_cost(
                    self.pts[res[pos]],
                    self.pts[res[pos + 1]],
                    self.pts[res[furthest - 1]],
                    self.pts[res[furthest]],
                    if furthest + 1 < n {
                        Some(self.pts[res[furthest + 1]])
                    } else {
                        None
                    },
                )
            };

            let mut min_diff = f64::MAX;
            let mut min_to = 0;
            let mut min_fto = 0;
            for to in 0..furthest - 1 {
                let mut diff = insert_at(to);
                for fto in furthest + 1..n {
                    diff += insert_cost(
                        self.pts[res[fto - 1]],
                        self.pts[res[to + 1]],
                        self.pts[res[furthest - 1]],
                        self.pts[res[fto]],
                        if fto + 1 < n {
                            Some(self.pts[res[fto + 1]])
                        } else {
                            None
                        },
                    );
                    if diff < min_diff {
                        min_diff = diff;
                        min_to = to;
                        min_fto = fto;
                    }
                }
            }

            // eprintln!(
            //     "furthest: {}, min_to: {}, min_fto: {}, min_diff: {}",
            //     furthest, min_to, min_fto, min_diff
            // );
            if min_diff < 0.0 {
                let path = res.drain(furthest..min_fto + 1).collect::<Vec<_>>();
                res.splice(min_to..min_to, path);
            } else {
                break;
            }
        }
        res
    }

    fn solve(&self) -> Vec<usize> {
        let mut mp = MovingPt::new(Pt::new(0, 0), Pt::new(0, 0));
        let mut vis = vec![false; self.pts.len()];
        let mut res = Vec::new();
        let n = self.pts.len();

        let ord = self.order();
        // let ord = self.order_greedy_with_angle();
        // let ord = self.order_stripe(80);

        let mut visited = 0;
        let mut loops = 0;
        loop {
            loops += 1;
            let mut visited_new = 0;
            for i in 0..n {
                if !vis[i] && self.pts[i] == mp.p {
                    vis[i] = true;
                    // eprintln!("visit {} (@loop {})", i, loops);
                }
                if vis[i] {
                    visited_new += 1;
                }
            }
            if visited_new > visited {
                for i in &ord {
                    let i = *i;
                    if vis[i] {
                        continue;
                    }
                    eprintln!(
                        "visited: {}/{} (@loop {}) (mp {:?}) (next {:?})",
                        visited_new, n, loops, mp, self.pts[i]
                    );
                    break;
                }
                visited = visited_new;
            }
            if visited == n {
                break;
            }

            for i in &ord {
                let i = *i;
                if vis[i] {
                    continue;
                }
                let a = self.calc_a(i, &vis, mp);
                mp.v = mp.v + a;
                mp.p = mp.p + mp.v;
                res.push(a.acc_as_tenkey());
                break;
            }
        }

        res
    }
}

fn main() {
    let env = Env::new(read_input());

    let res = env.solve();

    eprintln!("commands: {}", res.len());
    for r in res {
        print!("{}", r);
    }
    println!();
}
