// --- Day 6: Universal Orbit Map ---
//
// https://adventofcode.com/2019/day/6
//
//

use std::collections::HashMap;

type ObjectID = usize;

#[derive(Clone, Debug)]
struct Object {
    name: String,
    sats: HashMap<String, ObjectID>,
}

impl Object {
    fn new(name: String) -> Object {
        Object {
            name,
            sats: HashMap::new(),
        }
    }
    fn orbits(&self) -> usize {
        self.sats.len()
    }

    fn add_sat(&mut self, sat: ObjectID, obj_name: &str) {
        self.sats.insert(obj_name.to_string(), sat);
    }
}

#[derive(Debug)]
struct OrbitalMap {
    centre: ObjectID,
    objects: Vec<Object>,
}

impl OrbitalMap {
    fn from(data: Vec<&str>) -> OrbitalMap {
        let sats_obj_map: HashMap<&str, &str> = data
            .iter()
            .map(|x| {
                let p = x.splitn(2, ')').collect::<Vec<_>>();
                (p[1], p[0])
            })
            .collect();

        let mut obj_map = HashMap::new();
        let mut objs = vec![];
        for (sat, obj) in sats_obj_map {
            let satid = {
                if let Some(id) = obj_map.get(sat) {
                    *id
                } else {
                    let satobj = Object::new(sat.to_string());
                    objs.push(satobj);
                    obj_map.insert(sat, objs.len() - 1);
                    objs.len() - 1
                }
            };
            let objid = {
                if let Some(id) = obj_map.get(obj) {
                    *id
                } else {
                    let _obj = Object::new(obj.to_string());
                    objs.push(_obj);
                    obj_map.insert(obj, objs.len() - 1);
                    objs.len() - 1
                }
            };

            objs[objid].add_sat(satid, sat);
        }

        OrbitalMap {
            centre: *obj_map.get("COM").unwrap(),
            objects: objs,
        }
    }

    /// Returns reversed DFS path from target's parent to the start
    fn search(&self, target: &str, start: ObjectID) -> Vec<usize> {
        let mut path = HashMap::new();
        let mut stack = vec![];
        stack.push(start);
        let mut curr = None;
        while let Some(id) = stack.pop() {
            if self.objects[id].name == target {
                curr = Some(id);
                break;
            } else {
                for sid in self.objects[id].sats.values() {
                    path.insert(sid, id);
                    stack.push(*sid);
                }
            }
        }
        let mut res = vec![];
        while let Some(node) = curr {
            if let Some(ancestor) = path.get(&node) {
                res.push(*ancestor);
                curr = Some(*ancestor);
            } else {
                break;
            }
        }
        res
    }

    /// This is essentially an LCA (Least Common Ancestor)
    /// between the two positions and then counting the hops
    ///
    /// A crude LCA computation by finding the first common
    /// node on the reversed DFS paths of the two positions
    /// from the root (centre of mass).
    fn orbital_transfers(&self, to: &str, frm: &str) -> usize {
        let path1 = self.search(to, self.centre);
        let path2 = self.search(frm, self.centre);
        let mut cross = None;
        for p in &path1 {
            if path2.contains(p) {
                cross = Some(p);
                break;
            }
        }
        let cross = cross.unwrap();
        let mut count = 0;
        for p in &path1 {
            if p == cross {
                break;
            } else {
                count += 1;
            }
        }
        for p in &path2 {
            if p == cross {
                break;
            } else {
                count += 1;
            }
        }
        count
    }

    fn total_orbits(&self) -> usize {
        let mut objs = vec![(0, self.centre)];
        let mut count = 0;
        while let Some((lvl, oid)) = objs.pop() {
            count += self.objects[oid].orbits();
            for satid in self.objects[oid].sats.values() {
                count += lvl;
                objs.push((lvl + 1, *satid));
            }
        }
        count
    }
}
fn main() {
    let map = OrbitalMap::from(include_str!("input6.txt").lines().collect());
    println!("total orbits: {}", map.total_orbits());
    println!("orbital transfers: {}", map.orbital_transfers("YOU", "SAN"));
}

#[test]
fn test_orbital_map() {
    let map = OrbitalMap::from(vec![
        "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
    ]);
    println!("{:#?}", map.orbital_transfers("I", "L"));
    assert_eq!(map.total_orbits(), 42);
}
