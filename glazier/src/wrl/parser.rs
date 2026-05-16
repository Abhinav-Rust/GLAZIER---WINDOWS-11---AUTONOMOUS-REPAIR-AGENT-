use pest::Parser;
use pest::iterators::Pair;
use anyhow::{Result, anyhow};

use super::ast::*;

#[derive(pest_derive::Parser)]
#[grammar = "wrl/wrl.pest"]
pub struct WrlParser;

pub fn parse_wrl(input: &str) -> Result<Program> {
    let mut pairs = WrlParser::parse(Rule::program, input)?;
    let program_pair = pairs.next().ok_or_else(|| anyhow!("No program found"))?;

    let mut blocks = Vec::new();
    for pair in program_pair.into_inner() {
        if pair.as_rule() == Rule::EOI {
            break;
        }
        if pair.as_rule() == Rule::block {
            blocks.push(parse_block(pair.into_inner().next().unwrap())?);
        }
    }

    Ok(Program { blocks })
}

fn parse_block(pair: Pair<Rule>) -> Result<Block> {
    match pair.as_rule() {
        Rule::adhikara => Ok(Block::Adhikara(parse_adhikara(pair)?)),
        Rule::pratyaksha => Ok(Block::Pratyaksha(parse_pratyaksha(pair)?)),
        Rule::vyapti => Ok(Block::Vyapti(parse_vyapti(pair)?)),
        Rule::anumana => Ok(Block::Anumana(parse_anumana(pair)?)),
        Rule::abhava => Ok(Block::Abhava(parse_abhava(pair)?)),
        Rule::virodha => Ok(Block::Virodha(parse_virodha(pair)?)),
        Rule::tarka => Ok(Block::Tarka(parse_tarka(pair)?)),
        _ => Err(anyhow!("Expected block rule, found {:?}", pair.as_rule())),
    }
}

fn parse_adhikara(pair: Pair<Rule>) -> Result<AdhikaraNode> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let target = parse_component(inner.next().unwrap())?;

    let mut blocks = Vec::new();
    for p in inner {
        if p.as_rule() == Rule::block {
            blocks.push(parse_block(p.into_inner().next().unwrap())?);
        }
    }

    Ok(AdhikaraNode { name, target, blocks })
}

fn parse_component(pair: Pair<Rule>) -> Result<Component> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let param = inner.next().map(|p| p.as_str().to_string());
    Ok(Component { name, param })
}

fn parse_pratyaksha(pair: Pair<Rule>) -> Result<PratyakshaNode> {
    let mut inner = pair.into_inner();
    let observe_list = inner.next().unwrap();
    let via_list = inner.next().unwrap();

    let mut observe = Vec::new();
    for p in observe_list.into_inner() {
        observe.push(parse_property(p)?);
    }

    let mut via = Vec::new();
    for p in via_list.into_inner() {
        via.push(p.as_str().to_string());
    }

    Ok(PratyakshaNode { observe, via })
}

fn parse_property(pair: Pair<Rule>) -> Result<Property> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();

    let mut args = Vec::new();
    for p in inner {
        args.push(parse_property_arg(p)?);
    }

    Ok(Property { name, args })
}

fn parse_property_arg(pair: Pair<Rule>) -> Result<PropertyArg> {
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();

    if first.as_rule() == Rule::ident && inner.peek().is_some() {
        let name = first.as_str().to_string();
        let value = parse_property_val(inner.next().unwrap())?;
        Ok(PropertyArg::Named(name, Box::new(value)))
    } else {
        parse_property_val(first)
    }
}

fn parse_property_val(pair: Pair<Rule>) -> Result<PropertyArg> {
    match pair.as_rule() {
        Rule::string => Ok(PropertyArg::String(pair.into_inner().next().unwrap().as_str().to_string())),
        Rule::integer => Ok(PropertyArg::Integer(pair.as_str().parse()?)),
        Rule::action => Ok(PropertyArg::Action(Box::new(parse_action(pair)?))),
        Rule::ident => Ok(PropertyArg::Ident(pair.as_str().to_string())),
        _ => Err(anyhow!("Unexpected property val {:?}", pair.as_rule())),
    }
}

fn parse_vyapti(pair: Pair<Rule>) -> Result<VyaptiNode> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let condition_expr = inner.next().unwrap();
    let causal_conclusion = inner.next().unwrap().into_inner().next().unwrap().as_str().to_string();

    let mut witnessed = Vec::new();
    let next = inner.next().unwrap();
    let mut is_ident_list = false;
    if next.as_rule() == Rule::ident_list {
        is_ident_list = true;
        for p in next.clone().into_inner() {
            witnessed.push(p.as_str().to_string());
        }
    }

    let confidence_pair = if is_ident_list {
        inner.next().unwrap()
    } else {
        next
    };

    let confidence_str = if confidence_pair.as_rule() == Rule::confidence_level {
        if let Some(inner_pair) = confidence_pair.clone().into_inner().next() {
            inner_pair.as_str()
        } else {
            confidence_pair.as_str()
        }
    } else {
        confidence_pair.as_str()
    };
    let confidence = match confidence_str {
        "established" => Confidence::Established,
        "provisional" => Confidence::Provisional,
        "suspected" => Confidence::Suspected,
        _ => return Err(anyhow!("Unknown confidence level: {}", confidence_str)),
    };

    let whenever = parse_condition_expr(condition_expr)?;

    Ok(VyaptiNode {
        name,
        whenever,
        always: causal_conclusion,
        witnessed,
        confidence,
    })
}

fn parse_condition_expr(pair: Pair<Rule>) -> Result<Vec<(ConditionOp, Predicate)>> {
    let mut res = Vec::new();
    let mut current_op = ConditionOp::None;

    for p in pair.into_inner() {
        match p.as_str() {
            "AND" => current_op = ConditionOp::And,
            "OR" => current_op = ConditionOp::Or,
            _ => {
                res.push((current_op.clone(), parse_predicate(p)?));
                current_op = ConditionOp::None;
            }
        }
    }
    Ok(res)
}

fn parse_predicate(pair: Pair<Rule>) -> Result<Predicate> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::state_predicate => {
            let mut i = inner.into_inner();
            let property = parse_property(i.next().unwrap())?;
            let expected_state = i.next().map(|p| p.as_str().to_string());
            Ok(Predicate::State(StatePredicate { property, expected_state }))
        },
        Rule::temporal_context => {
            let mut i = inner.into_inner();
            let event = i.next().unwrap().as_str().to_string();
            let within = i.next().unwrap().as_str().to_string();
            Ok(Predicate::Temporal(TemporalContext { event, within }))
        },
        _ => Err(anyhow!("Unexpected predicate rule {:?}", inner.as_rule())),
    }
}

fn parse_anumana(pair: Pair<Rule>) -> Result<AnumanaNode> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let pratijnha = inner.next().unwrap().as_str().to_string();
    let hetu = parse_predicate(inner.next().unwrap())?;
    let udaharana = inner.next().unwrap().as_str().to_string();
    let upanaya = inner.next().unwrap().as_str().to_string();
    let nigamana = parse_action(inner.next().unwrap())?;

    Ok(AnumanaNode { name, pratijnha, hetu, udaharana, upanaya, nigamana })
}

fn parse_action(pair: Pair<Rule>) -> Result<Action> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();

    let mut args = Vec::new();
    if let Some(args_pair) = inner.next() {
        for p in args_pair.into_inner() {
            args.push(parse_action_arg(p)?);
        }
    }

    Ok(Action { name, args })
}

fn parse_action_arg(pair: Pair<Rule>) -> Result<ActionArg> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::action => Ok(ActionArg::Action(Box::new(parse_action(inner)?))),
        Rule::string => Ok(ActionArg::String(inner.into_inner().next().unwrap().as_str().to_string())),
        Rule::integer => Ok(ActionArg::Integer(inner.as_str().parse()?)),
        Rule::ident => Ok(ActionArg::Ident(inner.as_str().to_string())),
        _ => Err(anyhow!("Unexpected action arg {:?}", inner.as_rule())),
    }
}

fn parse_abhava(pair: Pair<Rule>) -> Result<AbhavaNode> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let pragabhava = parse_action(inner.next().unwrap())?;

    let mut dhvamsabhava = Vec::new();
    for p in inner.next().unwrap().into_inner() {
        dhvamsabhava.push(parse_action(p)?);
    }

    let atyantabhava = parse_action(inner.next().unwrap())?;
    let anyonyabhava = parse_action(inner.next().unwrap())?;

    Ok(AbhavaNode { name, pragabhava, dhvamsabhava, atyantabhava, anyonyabhava })
}

fn parse_tarka(pair: Pair<Rule>) -> Result<TarkaNode> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let assume = inner.next().unwrap().into_inner().next().unwrap().as_str().to_string();
    let predict = inner.next().unwrap().as_str().to_string();
    let test = parse_action(inner.next().unwrap())?;
    let if_contradicted_reject = inner.next().unwrap().as_str().to_string();
    let if_contradicted_action = parse_action(inner.next().unwrap())?;
    let if_confirmed_establish = inner.next().unwrap().as_str().to_string();
    let if_confirmed_action = parse_action(inner.next().unwrap())?;

    Ok(TarkaNode {
        name, assume, predict, test,
        if_contradicted_reject, if_contradicted_action,
        if_confirmed_establish, if_confirmed_action
    })
}

fn parse_virodha(pair: Pair<Rule>) -> Result<VirodhaNode> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let improving = inner.next().unwrap().as_str().to_string();
    let degrades = inner.next().unwrap().as_str().to_string();

    let guna_pair = inner.next().unwrap();
    let guna_str = if let Some(g) = guna_pair.clone().into_inner().next() { g.as_str() } else { guna_pair.as_str() };
    let guna = match guna_str {
        "tamas" => GunaType::Tamas,
        "rajas" => GunaType::Rajas,
        "sattva" => GunaType::Sattva,
        _ => return Err(anyhow!("Unknown guna: {}", guna_str)),
    };

    let mut upaya = Vec::new();
    for p in inner.next().unwrap().into_inner() {
        let mut i = p.into_inner();
        let utype_pair = i.next().unwrap();
        let utype_str = if let Some(u) = utype_pair.clone().into_inner().next() { u.as_str() } else { utype_pair.as_str() };
        let upaya_type = match utype_str {
            "sama" => UpayaType::Sama,
            "dana" => UpayaType::Dana,
            "bheda" => UpayaType::Bheda,
            "danda" => UpayaType::Danda,
            _ => return Err(anyhow!("Unknown upaya: {}", utype_str)),
        };
        let action = parse_action(i.next().unwrap())?;
        upaya.push(UpayaItem { upaya_type, action });
    }

    let mut upeksha_condition = None;
    let mut maya_path = None;

    for p in inner {
        if p.as_rule() == Rule::condition_expr {
            upeksha_condition = Some(parse_condition_expr(p)?);
        } else if p.as_rule() == Rule::action {
            maya_path = Some(parse_action(p)?);
        }
    }

    Ok(VirodhaNode {
        name, improving, degrades, guna, upaya, upeksha_condition, maya_path
    })
}
