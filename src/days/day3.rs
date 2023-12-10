use pest_derive::Parser as DeriveParser;
use pest::Parser;

#[derive(DeriveParser)]
#[grammar = "days/day3.pest"]
pub struct GamesParser;

struct Numbers {
    number: usize,
    top: usize,
    left: usize,
    bottom: usize,
    right: usize,
}

impl Numbers {
    fn in_range(&self,(x,y):(usize,usize)) -> Option<usize> {
        if self.top <=x && self.bottom >=x && self.left <=y && self.right >=y {
            Some(self.number)
        }
        else {
            None
        }
    }
}

fn parser(input: String, idx:usize, gear:bool) -> (Vec<Numbers>,Vec<(usize,usize)>) {
    let top = if idx == 0 {0} else {idx-1};
    let bottom = idx+1;
    let pairs = GamesParser::parse(Rule::schematic, &input)
        .unwrap()
        .next()
        .unwrap()
        .into_inner();
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    for pair in pairs{
        match pair.as_rule() {
            Rule::line => {
                let pairs = pair.into_inner();
                for pair in pairs {
                    match pair.as_rule() {
                        Rule::number => {
                            let number:usize = pair.as_str().parse().unwrap();
                            let span = pair.as_span();
                            let mut start:usize = span.start();
                            start = if start == 0 {0} else {start-1};
                            let end:usize = span.end();
                            numbers.push(Numbers{number,top,left:start,bottom,right:end});
                        },
                        Rule::symbol => {
                            if gear && pair.as_str() != "*" {
                                continue
                            }
                            let span = pair.as_span();
                            let start:usize = span.start();
                            symbols.push((idx,start));
                        },
                        _ => unreachable!()                        
                    };
                }
            },
            Rule::EOI => (),
            _ => unreachable!()
        };
    }

    (numbers,symbols)

}

pub fn solution1(input:Vec<String>)->usize{
    let mut value: usize = 0;
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    for (i,line) in input.iter().enumerate(){
        let (mut n,mut s) = parser(line.to_string(),i,false);
        numbers.append(&mut n);
        symbols.append(&mut s);
    }
    for number in numbers{
        for &symbol in &symbols{
            if let Some(v) = number.in_range(symbol){
                value += v;
                break
            }
        }
    }
    value
}

pub fn solution2(input:Vec<String>)->usize{
    let mut value: usize = 0;
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    for (i,line) in input.iter().enumerate(){
        let (mut n,mut s) = parser(line.to_string(),i,false);
        numbers.append(&mut n);
        symbols.append(&mut s);
    }
    for &symbol in &symbols{
        let mut val = Vec::new();
        for number in &numbers{
            if let Some(v) = number.in_range(symbol){
                val.push(v);
            }
        }
        if val.len() == 2 {
            value += val[0]*val[1];
        }
    }
    value
}