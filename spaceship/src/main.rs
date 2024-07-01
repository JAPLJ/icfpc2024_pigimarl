#![allow(dead_code)]
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    io::{self, Write},
    ops::{Add, Div, Mul, Sub},
};

use fixedbitset::FixedBitSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    fn from_tenkey(c: usize) -> Pt {
        match c {
            1 => Pt { x: -1, y: -1 },
            2 => Pt { x: 0, y: -1 },
            3 => Pt { x: 1, y: -1 },
            4 => Pt { x: -1, y: 0 },
            5 => Pt { x: 0, y: 0 },
            6 => Pt { x: 1, y: 0 },
            7 => Pt { x: -1, y: 1 },
            8 => Pt { x: 0, y: 1 },
            9 => Pt { x: 1, y: 1 },
            _ => panic!("invalid from_tenkey"),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MovingPt {
    p: Pt,
    v: Pt,
}

impl MovingPt {
    fn new(p: Pt, v: Pt) -> MovingPt {
        MovingPt { p, v }
    }

    fn from_sol(s: &str) -> MovingPt {
        let mut mp = MovingPt::new(Pt::new(0, 0), Pt::new(0, 0));
        for c in s.as_bytes() {
            let a = Pt::from_tenkey((*c - b'0') as usize);
            mp.v = mp.v + a;
            mp.p = mp.p + mp.v;
        }
        mp
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct St {
    mp: MovingPt,
    vis: usize,
    dis: usize,
}

impl Ord for St {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dis.cmp(&self.dis)
    }
}

impl PartialOrd for St {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

    fn calc_a(&self, target: usize, mp: MovingPt, lb: Option<i64>) -> (Pt, (usize, Pt)) {
        let mut min_t = usize::MAX;
        let mut min_a = Pt::new(0, 0);
        let mut total_v = Pt::new(0, 0);

        'outer: for total_t in lb.unwrap_or(1).. {
            let mut d = self.pts[target] - mp.p;
            d = d - mp.v * total_t;
            let mut first_a = None;
            let mut vv = Pt::new(0, 0);
            for t in 0..total_t {
                let rt = total_t - t;
                let a = Pt::new((d.x / rt).signum(), (d.y / rt).signum());
                vv = vv + a;
                if t == 0 {
                    first_a = Some(a);
                }
                d = d - a * rt;
                if d == Pt::new(0, 0) {
                    min_t = total_t as usize;
                    min_a = first_a.unwrap();
                    total_v = vv;
                    break 'outer;
                }
            }
        }

        (min_a, (min_t, total_v))
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
                // [23]
                // if loops == 1 && i == 39966 {
                //     continue;
                // }
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
            let mut min_to = 30;
            let mut min_fto = furthest;
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

            eprintln!(
                "furthest: {}, min_to: {}, min_fto: {}, min_diff: {}",
                furthest, min_to, min_fto, min_diff
            );
            // [23]
            // if loops == 2 || min_diff < 0.0 {
            if min_diff < 0.0 {
                let path = res.drain(furthest..min_fto + 1).collect::<Vec<_>>();
                res.splice(min_to..min_to, path);
            } else {
                break;
            }
        }
        res
    }

    fn solve15(&self) -> Vec<usize> {
        let mut q = VecDeque::new();
        let ini_vis = self
            .pts
            .iter()
            .position(|&p| p.x == 0 && p.y == 0)
            .map(|k| 1usize << k)
            .unwrap_or(0);
        q.push_back((MovingPt::new(Pt::new(0, 0), Pt::new(0, 0)), ini_vis));

        type St = (MovingPt, usize);
        let mut dist = HashMap::<St, usize>::new();
        let mut prev = HashMap::<St, St>::new();
        dist.insert(q[0], 0);

        let mut most_visited = 0;
        let mut rst = None;
        while let Some((mp, vis)) = q.pop_front() {
            let d = *dist.get(&(mp, vis)).unwrap();
            let count = vis.count_ones();
            if count + 3 <= most_visited {
                continue;
            }
            if count > most_visited {
                eprintln!(
                    "visited: {} -> {} (queue size {})",
                    most_visited,
                    count,
                    q.len()
                );
                most_visited = count;
            }
            if count as usize == self.pts.len() {
                rst = Some((mp, vis));
                break;
            }

            for ax in -1..=1 {
                for ay in -1..=1 {
                    let nv = mp.v + Pt::new(ax, ay);
                    let np = mp.p + nv;
                    let nmp = MovingPt::new(np, nv);
                    let nvis = vis
                        | self
                            .pts
                            .iter()
                            .position(|&p| p == np)
                            .map(|k| 1 << k)
                            .unwrap_or(0);
                    let nst = (nmp, nvis);
                    if dist.insert(nst, d + 1) == None {
                        q.push_back(nst);
                        prev.insert(nst, (mp, vis));
                    }
                }
            }
        }

        let mut rst = rst.unwrap();
        let mut res = vec![];
        while let Some(pst) = prev.get(&rst) {
            res.push((rst.0.v - pst.0.v).acc_as_tenkey());
            rst = *pst;
        }
        res.reverse();

        res
    }

    fn solve14_bitset(&self) -> Vec<usize> {
        let n = self.pts.len();
        let mut q = VecDeque::new();
        // let ini_mp = MovingPt::from_sol("22222228221179777779999999999999999999");
        let ini_vis = {
            let mut ini_vis = FixedBitSet::with_capacity(n);
            if let Some(k) = self.pts.iter().position(|&p| p.x == 0 && p.y == 0) {
                ini_vis.set(k, true);
            }
            // if let Some(k) = self.pts.iter().position(|&p| p == ini_mp.p) {
            //     ini_vis.set(k, true);
            // }
            ini_vis
        };
        // eprintln!("Start from {:?}", ini_mp);
        q.push_back(((MovingPt::new(Pt::new(0, 0), Pt::new(0, 0)), ini_vis), 0));
        // q.push_back(((ini_mp, ini_vis), 0));

        type St = (MovingPt, FixedBitSet);
        let mut dist = HashMap::<St, usize>::new();
        let mut prev = HashMap::<St, St>::new();
        dist.insert(q[0].0.clone(), 0);

        let mut most_visited = 0;
        let mut rst = None;
        while let Some(((mp, vis), d)) = q.pop_front() {
            let md = *dist.get(&(mp, vis.clone())).unwrap();
            if d != md {
                continue;
            }
            let count = vis.count_ones(..);
            if count + 2 <= most_visited {
                continue;
            }
            if count > most_visited {
                eprintln!(
                    "visited: {} -> {} (step {}, queue size {})",
                    most_visited,
                    count,
                    d,
                    q.len()
                );
                most_visited = count;
            }
            if count as usize == self.pts.len() {
                rst = Some((mp, vis));
                break;
            }

            for ax in -1..=1 {
                for ay in -1..=1 {
                    let nv = mp.v + Pt::new(ax, ay);
                    let np = mp.p + nv;
                    let nmp = MovingPt::new(np, nv);
                    let nvis = {
                        let mut nvis = vis.clone();
                        if let Some(k) = self.pts.iter().position(|&p| p == np) {
                            nvis.set(k, true);
                        }
                        nvis
                    };
                    let nst = (nmp, nvis);
                    if dist.insert(nst.clone(), d + 1) == None {
                        q.push_back((nst.clone(), d + 1));
                        prev.insert(nst, (mp, vis.clone()));
                    }
                }
            }
        }

        let mut rst = rst.unwrap();
        let mut res = vec![];
        while let Some(pst) = prev.get(&rst) {
            res.push((rst.0.v - pst.0.v).acc_as_tenkey());
            rst = pst.clone();
        }
        res.reverse();

        res
    }

    fn solve_search(&self) -> Vec<usize> {
        let ord = self.order();
        {
            let f = std::fs::File::create("sorted.txt").unwrap();
            let mut w = std::io::BufWriter::new(f);
            let mut prv = Pt::new(0, 0);
            for &i in &ord {
                let p = self.pts[i] - prv;
                prv = self.pts[i];
                write!(w, "{} {} ({} {})\n", p.x, p.y, prv.x, prv.y).unwrap()
            }
        }

        let z = Pt::new(0, 0);
        let mut q = VecDeque::new();
        let ini_vis = if self.pts[ord[0]] == z { 1 } else { 0 };
        q.push_back(St {
            mp: MovingPt::new(z, z),
            vis: ini_vis,
            dis: 0,
        });

        let mut dist = HashMap::<St, usize>::new();
        let mut prev = HashMap::<St, St>::new();
        dist.insert(q[0].clone(), 0);

        let mut dist_step = vec![0; ini_vis + 1];

        let mut most_visited = 0;
        let mut rst = None;
        while let Some(st) = q.pop_front() {
            let md = *dist.get(&st).unwrap();
            if st.dis != md {
                continue;
            }
            if st.vis + 2 <= most_visited {
                continue;
            }
            if st.vis > most_visited {
                while dist_step.len() <= st.vis {
                    dist_step.push(st.dis);
                }
                eprintln!(
                    "visited: {} -> {} ({:?}, step {}, queue size {})",
                    most_visited,
                    st.vis,
                    self.pts[ord[st.vis - 1]],
                    st.dis,
                    q.len()
                );
                most_visited = st.vis;
            }
            if st.vis == self.pts.len() {
                rst = Some(st);
                break;
            }

            for ax in -1..=1 {
                for ay in -1..=1 {
                    let nv = st.mp.v + Pt::new(ax, ay);
                    let np = st.mp.p + nv;
                    let nmp = MovingPt::new(np, nv);
                    let nvis = if self.pts[ord[st.vis]] == np {
                        st.vis + 1
                    } else {
                        st.vis
                    };
                    let nst = St {
                        mp: nmp,
                        vis: nvis,
                        dis: st.dis + 1,
                    };
                    if nvis > st.vis && dist_step.len() > nvis && nst.dis > dist_step[nvis] {
                        continue;
                    }
                    if nvis == st.vis
                        && dist_step.len() > nvis + 1
                        && nst.dis >= dist_step[nvis + 1]
                    {
                        continue;
                    }
                    if dist.insert(nst, nst.dis) == None {
                        q.push_back(nst);
                        prev.insert(nst, st);
                    }
                }
            }
        }

        let mut rst = rst.unwrap();
        let mut res = vec![];
        while let Some(pst) = prev.get(&rst) {
            res.push((rst.mp.v - pst.mp.v).acc_as_tenkey());
            rst = pst.clone();
        }
        res.reverse();

        res
    }

    // (v1, total, tm) -> accs
    fn precalc() -> HashMap<(i64, i64, usize), Vec<i64>> {
        fn srch(
            h: &mut HashMap<(i64, i64, usize), Vec<i64>>,
            a: &mut Vec<i64>,
            v: i64,
            tot: i64,
            tm: usize,
        ) {
            if tm >= 5 {
                return;
            }
            for dv in -1..=1 {
                let v = v + dv;
                let tot = tot + v;
                let tm = tm + 1;
                a.push(dv);
                if h.insert((v, tot, tm), a.clone()) == None {
                    srch(h, a, v, tot, tm);
                }
                a.pop();
            }
        }
        let mut h = HashMap::new();
        let mut a = vec![];
        srch(&mut h, &mut a, 0, 0, 0);
        h
    }

    fn solve_velocity_div(&self) -> Vec<usize> {
        // let ord = self.order();
        let ord = {
            let mut ord = vec![];
            for i in 0..self.pts.len() {
                ord.push(i);
            }
            ord
        };
        {
            let f = std::fs::File::create("sorted.txt").unwrap();
            let mut w = std::io::BufWriter::new(f);
            let mut prv = Pt::new(0, 0);
            for &i in &ord {
                let p = self.pts[i] - prv;
                prv = self.pts[i];
                write!(w, "{} {} ({} {})\n", p.x, p.y, prv.x, prv.y).unwrap()
            }
        }

        type St = (usize, Pt);
        let mut dist = HashMap::<St, usize>::new();
        let mut prev = HashMap::<St, St>::new();

        const MAXA: i64 = 9;
        const MAXT: usize = MAXA as usize;
        let accs = Env::precalc();

        let mut vs = vec![];
        let p0 = self.pts[ord[0]];
        for dx in -MAXA..=MAXA {
            for dy in -MAXA..=MAXA {
                for tm in 0..=MAXT {
                    let ax = accs.get(&(dx, p0.x, tm));
                    let ay = accs.get(&(dy, p0.y, tm));
                    match (ax, ay) {
                        (Some(_), Some(_)) => {
                            let st = (0usize, Pt::new(dx, dy));
                            dist.insert(st, tm);
                            prev.insert(st, (!0, Pt::new(0, 0)));
                            vs.push(st);
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        let n = self.pts.len();
        for i in 0..n - 1 {
            if i % 1000 == 0 {
                eprintln!("i={} sz={}", i, vs.len());
            }
            if vs.len() == 0 {
                panic!("awawa")
            }
            let mut nvs = HashSet::<St>::new();
            let p = self.pts[ord[i + 1]] - self.pts[ord[i]];
            for &(_, vel) in &vs {
                let cur_tm = *dist.get(&(i, vel)).unwrap();
                for dx in -MAXA..=MAXA {
                    for dy in -MAXA..=MAXA {
                        for tm in 1..=MAXT {
                            let p = p - vel * (tm as i64);
                            let ax = accs.get(&(dx, p.x, tm));
                            let ay = accs.get(&(dy, p.y, tm));
                            match (ax, ay) {
                                (Some(_), Some(_)) => {
                                    let st = (i + 1, vel + Pt::new(dx, dy));
                                    let ntm = cur_tm + tm;
                                    let pr = dist.get(&st);
                                    if pr.is_none() || pr.is_some_and(|&t| t > ntm) {
                                        nvs.insert(st);
                                        dist.insert(st, ntm);
                                        prev.insert(st, (i, vel));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            vs = nvs.into_iter().collect();
        }

        let mut best_tm = usize::MAX;
        let mut rst = None;
        for &(_, vel) in &vs {
            let tm = *dist.get(&(n - 1, vel)).unwrap();
            if best_tm > tm {
                best_tm = tm;
                rst = Some((n - 1, vel));
            }
        }

        let mut rst = rst.unwrap();
        let mut res = vec![];
        while let Some(&(pi, pvel)) = prev.get(&rst) {
            let pst = (pi, pvel);
            let i = rst.0;
            let fr = if i == 0 {
                Pt::new(0, 0)
            } else {
                self.pts[ord[i - 1]]
            };
            let to = self.pts[ord[i]];
            let tm = dist.get(&rst).unwrap() - dist.get(&pst).unwrap_or(&0);
            let p = to - fr - pvel * (tm as i64);
            let ax = accs.get(&(rst.1.x - pvel.x, p.x, tm)).unwrap();
            let ay = accs.get(&(rst.1.y - pvel.y, p.y, tm)).unwrap();
            for t in (0..tm).rev() {
                res.push(Pt::new(ax[t], ay[t]).acc_as_tenkey());
            }
            rst = pst;
        }

        res.reverse();
        res
    }

    fn solve_greed(&self) -> Vec<usize> {
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
                let (a, _) = self.calc_a(i, mp, None);
                mp.v = mp.v + a;
                mp.p = mp.p + mp.v;
                res.push(a.acc_as_tenkey());
                break;
            }
        }

        res
    }

    fn solve_greed2(&self) -> Vec<usize> {
        let mut mp = MovingPt::new(Pt::new(0, 0), Pt::new(0, 0));
        let mut vis = vec![false; self.pts.len()];
        let mut res = Vec::new();
        let n = self.pts.len();

        let ord = self.order();

        for lp in 0..n {
            eprintln!("loop {} (@{})", lp, res.len());
            let rem = {
                let mut rem = vec![];
                for i in 0..n {
                    if !vis[i] {
                        rem.push(i);
                    }
                }
                rem
            };

            let target = ord[lp];
            // let target = {
            //     let mut target = None;
            //     let mut min_time = usize::MAX;
            //     for &t in &rem {
            //         let (_, (tm, _)) = self.calc_a(t, mp);
            //         if min_time > tm {
            //             min_time = tm;
            //             target = Some(t);
            //         }
            //     }
            //     target.unwrap()
            // };

            let mut prev_tm1 = 0;
            while mp.p != self.pts[target] {
                let mut best_a = None;
                let mut min_time = usize::MAX;
                let mut min_tm1 = usize::MAX;
                for a in 1..=9 {
                    let acc = Pt::from_tenkey(a);
                    let mut mp = mp;
                    mp.v = mp.v + acc;
                    mp.p = mp.p + mp.v;
                    if mp.p == self.pts[target] {
                        best_a = Some(a);
                        break;
                    }
                    let (_, (tm1, dv)) =
                        self.calc_a(target, mp, Some((prev_tm1 as i64 - 3).max(1)));
                    mp.p = self.pts[target];
                    mp.v = mp.v + dv;
                    let nearest = if rem.len() < 2 {
                        0
                    } else {
                        let (_, (tm2, _)) = self.calc_a(ord[lp + 1], mp, None);
                        tm2
                    };
                    if tm1 + nearest < min_time {
                        min_time = tm1 + nearest;
                        min_tm1 = tm1;
                        best_a = Some(a);
                    }
                }
                prev_tm1 = min_tm1;
                let best_a = best_a.unwrap();
                res.push(best_a);
                mp.v = mp.v + Pt::from_tenkey(best_a);
                mp.p = mp.p + mp.v;
            }
            vis[target] = true;
        }

        res
    }
}

fn main() {
    let env = Env::new(read_input());

    let res = env.solve_greed2();

    eprintln!("commands: {}", res.len());
    for r in res {
        print!("{}", r);
    }
    println!();
}
