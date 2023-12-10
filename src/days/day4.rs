use pest_derive::Parser as DeriveParser;
use pest::Parser;

#[derive(DeriveParser)]
#[grammar = "days/day4.pest"]
pub struct CardsParser;

fn parser(input: String) -> usize {
    let mut pairs = CardsParser::parse(Rule::card, &input)
        .unwrap()
        .next()
        .unwrap()
        .into_inner();
    let _id:u32 = pairs.next().unwrap().as_str().parse().unwrap();
    let mut win:Vec<u32> = Vec::new();
    let mut mine:Vec<u32> = Vec::new();
    for pair in pairs{
        match pair.as_rule() {
            Rule::win => {
                let pairs = pair.into_inner();
                for pair in pairs{
                    match pair.as_rule() {
                        Rule::number => win.push(pair.as_str().parse().unwrap()),
                        _ => unreachable!(),
                    }
                }
            },
            Rule::mine => {
                let pairs = pair.into_inner();
                for pair in pairs{
                    match pair.as_rule() {
                        Rule::number => mine.push(pair.as_str().parse().unwrap()),
                        _ => unreachable!(),
                    }
                }
            },
            Rule::EOI => (),
            _ => unreachable!()
        };
    }
    win.sort_unstable();
    mine.sort_unstable();
    let mut pow = 0;
    for w in win{
        if mine.binary_search(&w).is_ok() {
            pow +=1;
        }
    }
    pow
}

pub fn solution1(input:Vec<String>)->usize{
    let mut value: usize = 0;
    for line in input{
        let pow = parser(line);
        if pow !=0  {
            value += 2_usize.pow((pow-1).try_into().unwrap())
        }
    }
    value
}

pub fn solution2(input:Vec<String>)->usize{
    let mut rep:Vec<usize> = vec![1;input.len()];
    for (i,line) in input.iter().enumerate(){
        let val = parser(line.to_string());
        if val !=0 {
            for j in i+1..i+1+val{
                if j < rep.len() {
                    rep[j]+=rep[i];
                }
                else {
                    break
                }
            }
        }
    }
    rep.iter().sum()
}