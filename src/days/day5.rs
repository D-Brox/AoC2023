use std::collections::VecDeque;

use pest_derive::Parser as DeriveParser;
use pest::Parser;

#[derive(DeriveParser)]
#[grammar = "days/day5.pest"]
pub struct AlmanacParser;

struct Range{
    src: u64,
    dst: u64,
    range: u64,
}

type RangePair = (Vec<(u64,u64)>,Vec<(u64,u64)>);

impl Range {
    fn src_to_dst(&self,src:u64) -> Option<u64> {
        if src >= self.src && src < self.src + self.range {
            Some(self.dst + src - self.src)
        }
        else {
            None
        }
    }
    fn range_match(&self,(src,range):(u64,u64)) -> RangePair {
        let mut matched = Vec::new();
        let mut unmatched = Vec::new();
        //  (  ) [  ] or [  ] (  )
        if src + range <= self.src || src >= self.src + self.range {
            unmatched.push((src,range));
            return (matched,unmatched);
        }
        // [ (  ) ]
        else if src >= self.src && src + range <= self.src + self.range {
            matched.push((self.dst + src - self.src,range));
            return (matched,unmatched)
        }
        // ( [
        if src < self.src{
            unmatched.push((src,self.src - src));
        }
        // ] )
        if src + range > self.src + self.range {
            unmatched.push((self.src + self.range,src + range - self.src - self.range));
        }
        // ( ]
        if self.src <= src && self.src + self.range <= src + range{
            matched.push((self.dst + src - self.src,self.src + self.range - src))
        } 
        // [ )
        if src <= self.src && src + range <= self.src + self.range {
            matched.push((self.dst,src + range - self.src)); 
        }
        // [ ]
        if  src < self.src && src + range > self.src + self.range {
            matched.push((self.dst,self.range));
        } 
        (matched,unmatched)
    }
}

// type 

fn parse(input:String) -> (Vec<u64>,Vec<Vec<Range>>){
    let mut pairs = AlmanacParser::parse(Rule::almanac, &input)
        .unwrap()
        .next()
        .unwrap()
        .into_inner();
    let mut seeds:Vec<u64> = Vec::new();
    
    for pair in pairs.next().unwrap().into_inner(){
        match pair.as_rule() {
            Rule::number => seeds.push(pair.as_str().parse().unwrap()),
            _ => unreachable!("pairwise"),
        }
    }
    let mut maps = Vec::new();
    for pair in pairs {
        let mut map = Vec::new();
        match pair.as_rule() {
            Rule::map => {
                let pairs = pair.into_inner();
                for pair in pairs {
                    match pair.as_rule() {
                        Rule::range => {
                            let mut pairs = pair.into_inner();
                            let dst = pairs.next().unwrap().as_str().parse().unwrap();
                            let src = pairs.next().unwrap().as_str().parse().unwrap();
                            let range = pairs.next().unwrap().as_str().parse().unwrap();
                            map.push(Range{src,dst,range})
                        },
                        _ => unreachable!()
                    }
                }
            },
            Rule::EOI => (),                                
            _ => unreachable!(),
        }
        maps.push(map)
    } 
    (seeds,maps)
}

pub fn solution1(input:Vec<String>) -> u64{
    let input = input.join("\n");
    let mut output= Vec::new();
    let (seeds,maps) = parse(input);

    for seed in seeds {
        let mut mapped = seed;
        for map in &maps {
            for range in map{
                if let Some(dst) = range.src_to_dst(mapped){
                    mapped = dst;
                    break
                }
            }
        }
        output.push(mapped);
    }

    *output.iter().min().unwrap()
}

pub fn solution2(input:Vec<String>) -> u64{
    let input = input.join("\n");
    let (seeds,maps) = parse(input);
    let mut mapped:VecDeque<(u64,u64)> = seeds.chunks(2).map(|s|if let [a,b] = s {(*a,*b)} else {(0_u64,0_u64)}).collect();
    let mut remapped = Vec::new();
    for map in &maps {
        'outer: while let Some(mut r) = mapped.pop_front() {
            for range in map {
                let ( mut matched, unmatched) = range.range_match(r);
                if !matched.is_empty() {
                    remapped.append(&mut matched);
                    let _ = unmatched.iter().map(|u| mapped.push_front(*u)).collect::<Vec<()>>();
                    continue 'outer;
                }
                else {
                    r = unmatched[0];
                }
            }
            remapped.push(r);
        }
        mapped = remapped.into();
        remapped = Vec::new();
    }

    mapped.iter().map(|(s,_)|s)
        .min()
        .unwrap()
        .to_owned()
}