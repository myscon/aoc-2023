//! YEAR:   2023
//! DAY:    10

use crate::prelude::*;
use crate::regex::regex;
use std::collections::HashMap;

impl Answers for Day {
    fn part_one(&mut self) -> String {
        let mut maze = Maze::new(&mut self.reader);
        maze.generate_loop();
        maze.length.div_ceil(2).to_string()
    }

    fn part_two(&mut self) -> String {
        let mut maze = Maze::new(&mut self.reader);
        maze.generate_loop();
        maze.calculate_area();
        maze.area.to_string()
    }
}

struct Maze {
    maze: Vec<Vec<Tile>>,
    start: Coord,
    length: usize,
    area: usize,
}

trait Paths {
    fn new(reader: &mut BufReader<File>) -> Self;
    fn generate_loop(&mut self);
    fn calculate_area(&mut self);
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    x: i32,
    y: i32,
    t: bool,
    l: char,
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Paths for Maze {
    fn new(reader: &mut BufReader<File>) -> Self {
        let mut buffer = String::new();
        let _ = reader.read_line(&mut buffer);
        let mut maze = vec![];
        let mut start = Coord { y: 0, x: 0 };
        let mut map = HashMap::new();
        // this made me question whether the regex crate was worth it...
        let pattern = regex("s");
        // I suppose I could have left the maze as is and use this to map directions but
        // I didn't know how I would solve part two and didn't know what I would need
        map.insert(
            '|',
            Tile {
                y: 0,
                x: 0,
                t: false,
                l: '|',
            },
        );
        map.insert(
            '-',
            Tile {
                y: 0,
                x: 0,
                t: false,
                l: '-',
            },
        );
        map.insert(
            'L',
            Tile {
                y: -1,
                x: 1,
                t: false,
                l: 'L',
            },
        );
        map.insert(
            'J',
            Tile {
                y: -1,
                x: -1,
                t: false,
                l: 'J',
            },
        );
        map.insert(
            '7',
            Tile {
                y: 1,
                x: -1,
                t: false,
                l: '7',
            },
        );
        map.insert(
            'F',
            Tile {
                y: 1,
                x: 1,
                t: false,
                l: 'F',
            },
        );
        map.insert(
            '.',
            Tile {
                y: 2,
                x: 2,
                t: false,
                l: '.',
            },
        );
        while buffer.len() > 0 {
            if let Some(start_match) = pattern.find(&buffer).unwrap() {
                start = Coord {
                    y: maze.len() as i32,
                    x: start_match.start() as i32,
                };
            }
            let maze_chars = buffer
                .trim()
                .chars()
                .map(|s| {
                    // I could make this a little more dynamic but whatever
                    *map.get(&s).unwrap_or(&Tile {
                        x: 2,
                        y: 2,
                        t: true,
                        l: '7',
                    })
                })
                .collect::<Vec<Tile>>();
            maze.push(maze_chars);
            buffer.clear();
            _ = reader.read_line(&mut buffer);
        }
        Maze {
            maze: maze,
            start: start,
            length: 0,
            area: 0,
        }
    }

    fn generate_loop(&mut self) {
        let prot_moves = vec![
            Coord { y: 1, x: 0 },
            Coord { y: 0, x: 1 },
            Coord { y: -1, x: 0 },
            Coord { y: 0, x: -1 },
        ];
        let mut curr_node = self.start;
        let mut prev_move = Coord { y: 0, x: 0 };
        let mut steps = 0;

        for prot_move in &prot_moves {
            let next_node = Coord {
                y: curr_node.y + prot_move.y,
                x: curr_node.x + prot_move.x,
            };
            
            let next_move = self.maze[next_node.y as usize][next_node.x as usize];
            if (next_move.y + prot_move.y).abs() < 2 && (next_move.x + prot_move.x).abs() < 2 {
                if next_move.l == '|' && prot_move.x > 0{
                    continue;
                }
                if next_move.l == '-' && prot_move.y > 0 {
                    continue;
                }
                curr_node = next_node;
                prev_move = prot_move.clone();
                steps += 1;
                break;
            }
        }
        '_loop: while curr_node != self.start {
            let prot_move = &mut self.maze[curr_node.y as usize][curr_node.x as usize];
            prot_move.t = true;
            let curr_move = Coord {
                y: prev_move.y + prot_move.y,
                x: prev_move.x + prot_move.x,
            };
            let next_node = Coord {
                y: curr_node.y + curr_move.y,
                x: curr_node.x + curr_move.x,
            };
            curr_node = next_node;
            prev_move = curr_move;
            steps += 1;
            if curr_node == self.start {
                self.length = steps;
            }
        }
    }

    // I googled how to to find out if a point is in a polygon or not so I guess give me an
    // asterisk. Should I take a math class or something?
    fn calculate_area(&mut self) {
        let vert = vec!['F', 'J', 'L', '7', '|'];
        let mut sqft = 0;
        for tiles in &self.maze {
            for i in 0..tiles.len() {
                let mut left = 0;
                if !tiles[i].t {
                    let mut prev = 'N';
                    for l in 0..i {
                        if vert.contains(&tiles[l].l) && tiles[l].t {
                            if !(prev == 'F' && tiles[l].l == 'J')
                                && !(prev == 'L' && tiles[l].l == '7')
                            {
                                left += 1;
                            }
                            prev = tiles[l].l;
                        }
                    }
                }
                if left % 2 > 0 {
                    sqft += 1;
                }
            }
        }
        self.area = sqft;
    }
}
