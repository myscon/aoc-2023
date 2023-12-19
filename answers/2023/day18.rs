//! YEAR:   2023
//! DAY:    18

use crate::prelude::*;

impl Answers for Day {
    fn part_one(&mut self) -> String {
        let mut lagoon = Lagoon::new(&self.input);
        lagoon.shoelace_area().to_string()
    }

    fn part_two(&mut self) -> String {
        let mut lagoon = Lagoon::new(&self.input);
        lagoon.parse_colors();
        lagoon.shoelace_area().to_string()
    }
}

struct Lagoon<'a> {
    trenches: Vec<Trench<'a>>,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Trench<'a> {
    coord: Coord,
    direction: char,
    distance: usize,
    color: &'a str,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Coord {
    x: i64,
    y: i64,
}

impl<'a> Lagoon<'a> {
    fn new(input: &'a str) -> Self {
        let mut x_cursor = 0;
        let mut y_cursor = 0;
        let mut trenches = vec![];
        for line in input.lines() {
            let mut split = line.split(" ");
            trenches.push(dig_trench(
                &mut x_cursor,
                &mut y_cursor,
                split.next().unwrap().chars().next().unwrap(),
                split.next().unwrap().parse::<i64>().unwrap(),
                split.next().unwrap(),
            ))
        }
        Lagoon { trenches: trenches }
    }

    #[allow(unused)]
    fn orient_positive(&mut self) {
        let min_x = self
            .trenches
            .iter()
            .map(|p| p.coord.x)
            .fold(i64::MAX, i64::min);
        let min_y = self
            .trenches
            .iter()
            .map(|p| p.coord.y)
            .fold(i64::MAX, i64::min);
        if min_x < 0 {
            self.trenches.iter_mut().for_each(|t| {
                t.coord.x -= min_x;
            });
        }
        if min_y < 0 {
            self.trenches.iter_mut().for_each(|t| {
                t.coord.y -= min_y;
            });
        }
    }

    fn build_graph(&self) -> Vec<Vec<char>> {
        let mut graph = vec![vec![]];
        let mut start = self.trenches.last().unwrap();
        for trench in &self.trenches {
            let x_diff = 1 + trench.coord.x - graph[0].len() as i64;
            let y_diff = 1 + trench.coord.y - graph.len() as i64;
            if y_diff > 0 {
                (0..y_diff).for_each(|_| graph.push(vec!['.'; graph[0].len()]));
            }
            if x_diff > 0 {
                graph
                    .iter_mut()
                    .for_each(|l| (0..x_diff).for_each(|_| l.push('.')))
            }
            if start.coord.x == trench.coord.x {
                (start.coord.y.min(trench.coord.y)..=start.coord.y.max(trench.coord.y))
                    .for_each(|n| graph[n as usize][start.coord.x as usize] = trench.direction)
            }
            if start.coord.y == trench.coord.y {
                (start.coord.x.min(trench.coord.x)..=start.coord.x.max(trench.coord.x))
                    .for_each(|n| graph[start.coord.y as usize][n as usize] = trench.direction);
                // This line is so I can do the vert count for the area calculations.
                graph[start.coord.y as usize][start.coord.x as usize] = start.direction;
            }
            start = trench;
        }
        graph
    }

    // This worked well for part one but the graph built is bigger than usize::MAX.
    #[allow(unused)]
    fn calculate_area(&self) -> i64 {
        let graph = self.build_graph();
        let mut area = 0;
        graph.iter().rev().for_each(|l| {
            let mut trench_points = 0;
            let mut prev_vert = '.';
            l.iter().for_each(|&c| match c {
                'U' | 'D' => {
                    if prev_vert != c {
                        trench_points += 1;
                        prev_vert = c;
                    }
                    area += 1
                }
                '.' => {
                    if trench_points % 2 != 0 {
                        area += 1;
                    }
                }
                'L' | 'R' => {
                    area += 1;
                }
                _ => unreachable!(),
            });
        });
        area
    }

    fn parse_colors(&mut self) {
        let mut x_cursor = 0;
        let mut y_cursor = 0;
        for trench in &mut self.trenches {
            *trench = dig_trench(
                &mut x_cursor,
                &mut y_cursor,
                match &trench.color[trench.color.len() - 2..trench.color.len() - 1] {
                    "0" => 'R',
                    "1" => 'D',
                    "2" => 'L',
                    "3" => 'U',
                    _ => unreachable!(),
                },
                i64::from_str_radix(&trench.color[2..trench.color.len() - 2], 16).unwrap(),
                trench.color,
            );
        }
    }

    #[allow(unused)]
    fn shoelace_area(&mut self) -> i64 {
        let mut area = 0;
        let mut line = 0;
        let mut start = self.trenches.last().unwrap();
        for trench in &self.trenches {
            line += trench.distance as i64;
            area += start.coord.wedge_product(&trench.coord);
            start = trench;
        }
        // I was banging my head against the wall because the equation above matched the wiki. 
        // However, it turns out the line isn't as simple as the trenches.len() because the
        // segments overlap. I think I could elimate some of the preprocessing steps because of this.
        (area / 2).abs() + line / 2 + 1
    }

    // This didn't work at all and ended up being completely uneeded because I realize
    // that the coordinates are already sorted in either cc or c direction.
    #[allow(unused)]
    fn counter_clockwise_sort(&mut self) {
        let centroid = self.centroid();
        self.trenches.sort_by(|a, b| {
            let angle_a = a.coord.angle(&centroid);
            let angle_b = b.coord.angle(&centroid);
            angle_b.partial_cmp(&angle_a).unwrap()
        });
    }

    fn centroid(&self) -> Coord {
        let sum: Coord = self
            .trenches
            .iter()
            .fold(Coord { x: 0, y: 0 }, |acc, p| Coord {
                x: acc.x + p.coord.x,
                y: acc.y + p.coord.y,
            });

        Coord {
            x: sum.x / self.trenches.len() as i64,
            y: sum.y / self.trenches.len() as i64,
        }
    }
}

impl Coord {
    #[allow(unused)]
    fn wedge_product(&self, other: &Coord) -> i64 {
        (self.x * other.y) - (self.y * other.x)
    }
    // I'm going to be real with you. I watched an hour and half of youtube geometry.
    // waste of time tbh
    fn angle(&self, reference: &Coord) -> f64 {
        let y_diff = (self.y - reference.y) as f64;
        let x_diff = (self.x - reference.x) as f64;
        y_diff.atan2(x_diff)
    }
}

fn dig_trench<'a>(
    x_cursor: &mut i64,
    y_cursor: &mut i64,
    direction: char,
    distance: i64,
    color: &'a str,
) -> Trench<'a> {
    match direction {
        'U' => {
            *y_cursor += distance;
        }
        'D' => {
            *y_cursor -= distance;
        }
        'L' => {
            *x_cursor -= distance;
        }
        'R' => {
            *x_cursor += distance;
        }
        _ => unreachable!(),
    };
    Trench {
        coord: Coord {
            x: *x_cursor,
            y: *y_cursor,
        },
        direction: direction,
        distance: distance as usize,
        color: color,
    }
}
