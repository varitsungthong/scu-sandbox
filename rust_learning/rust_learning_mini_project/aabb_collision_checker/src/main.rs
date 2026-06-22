#[derive(Debug)]

struct AABB {
    min_x : i32 , max_x : i32 ,
    min_y : i32 , max_y : i32 ,
    min_z : i32 , max_z : i32 ,
}

impl AABB {
    fn intersect (&self , other : &AABB) -> bool {
        (self.min_x <= other.max_x && self.max_x >= other.min_x) &&
        (self.min_y <= other.max_y && self.max_y >= other.min_y) &&
        (self.min_z <= other.max_z && self.max_z >= other.min_z) 
    }

}
#[derive(Debug)]
struct PrototypeDef {
    id :u16 ,
    name : String ,
}
#[derive(Debug)]
struct Instance {
    id : u32 ,
    prototype_id : u16,
    aabb : AABB,
}


fn main() {
    let prototypes =  [
        PrototypeDef {
        id : 0,
        name : ("Beam").to_string( ),
    }
    ];

    let instances = [
        Instance {
            id : 0,
            prototype_id : 0,
            aabb : AABB { min_x : 0 , max_x : 10 ,min_y : 0 , max_y : 10 , min_z : 0 , max_z : 10 ,}
        },
        Instance {
            id : 1,
            prototype_id : 0,
            aabb : AABB { min_x : 5 , max_x : 7 ,min_y : 0 , max_y : 15 , min_z : 2 , max_z : 5 ,}
        },
        Instance{
            id : 2,
            prototype_id : 0,
            aabb : AABB { min_x : 15 , max_x : 20 ,min_y : 15 , max_y : 2147483647 , min_z : 0 , max_z : 1010,}
        },
    ];
    dbg!(&instances);
    dbg!(&prototypes);

    println!("Loaded Prototype : {}, ID : {}", prototypes[0].name, prototypes[0].id);
    println!("instances A id : {}, prototype_ID : {}", instances[0].id, instances[0].prototype_id);
    println!("instances B id : {}, prototype_ID : {}", instances[1].id, instances[1].prototype_id);
    println!("instances C id : {}, prototype_ID : {}", instances[2].id, instances[2].prototype_id);

    let a = &instances[0];
    let b = &instances[1];
    let c = &instances[2];

    let mut clashes = 0;
    if a.aabb.intersect(&b.aabb) {
        println!("Clash detected! A and B");clashes += 1
    }
    if b.aabb.intersect(&c.aabb) {
        println!("Clash detected! B and C");clashes += 1
    }
    if a.aabb.intersect(&c.aabb) {
        println!("Clash detected! A and C");clashes += 1
    }
    if clashes == 0 {
         println!("Clear")
    }
    

}
