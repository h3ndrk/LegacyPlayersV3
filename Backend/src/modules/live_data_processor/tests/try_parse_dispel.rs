use crate::modules::armory::Armory;
use crate::modules::live_data_processor::domain_value::Unit as DomainValueUnit;
use crate::modules::live_data_processor::domain_value::{AuraApplication, Creature, HitType, SpellCast};
use crate::modules::live_data_processor::domain_value::{Event, EventType};
use crate::modules::live_data_processor::dto::UnAura;
use crate::modules::live_data_processor::dto::Unit as DtoUnit;
use crate::modules::live_data_processor::tools::server::try_parse_dispel;
use std::collections::HashMap;

#[test]
fn test_aura_application_aggregation_predicate() {
    // setup dependencies
    let armory = Armory::default();

    let mut summons: HashMap<u64, u64> = HashMap::new();
    summons.insert(0xF130 << 48, 0xF130 << 48);

    // testing predicate in line 35 of dispel.rs: the following clauses exist in the predicate:
    //  a = aura_application_event_ids.len() < dispel.un_aura_amount as usize
    //  b = event.subject == target (dispel.target)
    //  c = aura_application.spell_id == dispel.target_spell_id
    // Correlated Active Clause Coverage (CACC) yields the following tests:
    //  a*b*c, !a*b*c, a*!b*c, a*b*!c

    let result_a_b_c = try_parse_dispel(
        &UnAura {
            un_aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            // right part of b:
            target: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            un_aura_spell_id: 42,
            // right part of c:
            target_spell_id: 42,
            // right part of a:
            un_aura_amount: 1,
        },
        // amount of AuraApplications is left part of a:
        &[
            Event {
                id: 42,
                timestamp: 1337,
                // left part of b:
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::AuraApplication(AuraApplication {
                    caster: DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    }),
                    stack_amount: 42,
                    // left part of c:
                    spell_id: 42,
                }),
            },
            Event {
                id: 42,
                timestamp: 1337,
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::SpellCast(SpellCast {
                    victim: Some(DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    })),
                    hit_type: HitType::Evade,
                    spell_id: Some(42),
                    damage: vec![],
                    heal: vec![],
                    threat: vec![],
                }),
            },
        ],
        1337,
        1337,
        &armory,
        42,
        &summons,
    );
    assert!(result_a_b_c.is_ok());
    let result_a_b_c = result_a_b_c.unwrap();
    assert_eq!(result_a_b_c.0, 42);
    assert_eq!(result_a_b_c.1.len(), 1);
    assert_eq!(result_a_b_c.1[0], 42);

    let result_not_a_b_c = try_parse_dispel(
        &UnAura {
            un_aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            // right part of b:
            target: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            un_aura_spell_id: 42,
            // right part of c:
            target_spell_id: 42,
            // right part of a:
            un_aura_amount: 0,
        },
        // amount of AuraApplications is left part of a:
        &[
            Event {
                id: 42,
                timestamp: 1337,
                // left part of b:
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::AuraApplication(AuraApplication {
                    caster: DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    }),
                    stack_amount: 42,
                    // left part of c:
                    spell_id: 42,
                }),
            },
            Event {
                id: 42,
                timestamp: 1337,
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::SpellCast(SpellCast {
                    victim: Some(DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    })),
                    hit_type: HitType::Evade,
                    spell_id: Some(42),
                    damage: vec![],
                    heal: vec![],
                    threat: vec![],
                }),
            },
        ],
        1337,
        1337,
        &armory,
        42,
        &summons,
    );
    assert!(result_not_a_b_c.is_ok());
    let result_not_a_b_c = result_not_a_b_c.unwrap();
    assert_eq!(result_not_a_b_c.0, 42);
    assert_eq!(result_not_a_b_c.1.len(), 0);

    let result_a_not_b_c = try_parse_dispel(
        &UnAura {
            un_aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            // right part of b:
            target: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            un_aura_spell_id: 42,
            // right part of c:
            target_spell_id: 42,
            // right part of a:
            un_aura_amount: 1,
        },
        // amount of AuraApplications is left part of a:
        &[
            // one event which gets accepted
            Event {
                id: 42,
                timestamp: 1337,
                // left part of b:
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::AuraApplication(AuraApplication {
                    caster: DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    }),
                    stack_amount: 42,
                    // left part of c:
                    spell_id: 42,
                }),
            },
            // one event with non-matching subject creature id
            Event {
                id: 42,
                timestamp: 1337,
                // left part of b:
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: (0xF130 << 48) + 1,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::AuraApplication(AuraApplication {
                    caster: DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    }),
                    stack_amount: 42,
                    // left part of c:
                    spell_id: 42,
                }),
            },
            Event {
                id: 42,
                timestamp: 1337,
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::SpellCast(SpellCast {
                    victim: Some(DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    })),
                    hit_type: HitType::Evade,
                    spell_id: Some(42),
                    damage: vec![],
                    heal: vec![],
                    threat: vec![],
                }),
            },
        ],
        1337,
        1337,
        &armory,
        42,
        &summons,
    );
    assert!(result_a_not_b_c.is_ok());
    let result_a_not_b_c = result_a_not_b_c.unwrap();
    assert_eq!(result_a_not_b_c.0, 42);
    assert_eq!(result_a_not_b_c.1.len(), 1);
    assert_eq!(result_a_not_b_c.1[0], 42);

    let result_a_b_not_c = try_parse_dispel(
        &UnAura {
            un_aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            // right part of b:
            target: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            un_aura_spell_id: 42,
            // right part of c:
            target_spell_id: 42,
            // right part of a:
            un_aura_amount: 1,
        },
        // amount of AuraApplications is left part of a:
        &[
            // one event which gets accepted
            Event {
                id: 42,
                timestamp: 1337,
                // left part of b:
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::AuraApplication(AuraApplication {
                    caster: DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    }),
                    stack_amount: 42,
                    // left part of c:
                    spell_id: 42,
                }),
            },
            // one event with non-matching event's spell id
            Event {
                id: 42,
                timestamp: 1337,
                // left part of b:
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::AuraApplication(AuraApplication {
                    caster: DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    }),
                    stack_amount: 42,
                    // left part of c:
                    spell_id: 43,
                }),
            },
            Event {
                id: 42,
                timestamp: 1337,
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::SpellCast(SpellCast {
                    victim: Some(DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    })),
                    hit_type: HitType::Evade,
                    spell_id: Some(42),
                    damage: vec![],
                    heal: vec![],
                    threat: vec![],
                }),
            },
        ],
        1337,
        1337,
        &armory,
        42,
        &summons,
    );
    assert!(result_a_b_not_c.is_ok());
    let result_a_b_not_c = result_a_b_not_c.unwrap();
    assert_eq!(result_a_b_not_c.0, 42);
    assert_eq!(result_a_b_not_c.1.len(), 1);
    assert_eq!(result_a_b_not_c.1[0], 42);
}

#[test]
fn test_aura_application_success_return_predicate() {
    // setup dependencies
    let armory = Armory::default();

    let mut summons: HashMap<u64, u64> = HashMap::new();
    summons.insert(1, 1);
    // testing predicate in line 51 of dispel.rs: the following clauses exist in the predicate:
    //  a = aura_application_event_ids.len() == dispel.un_aura_amount as usize
    //  b = !aura_application_event_ids.is_empty()
    //  c = next_timestamp as i64 - timestamp as i64 > 10
    // the predicate is: a + b*c
    //  # | a | b | c | b*c | a + b*c
    // ---+---+---+---+-----+---------
    //  1 | 0 | 0 | 0 |  0  |    0
    //  2 | 1 | 0 | 0 |  0  |    1
    //  3 | 0 | 1 | 0 |  0  |    0
    //  4 | 1 | 1 | 0 |  0  |    1
    //  5 | 0 | 0 | 1 |  0  |    0
    //  6 | 1 | 0 | 1 |  0  |    1
    //  7 | 0 | 1 | 1 |  1  |    1
    //  8 | 1 | 1 | 1 |  1  |    1

    // Correlated Active Clause Coverage (CACC) yields the following tests:
    // for major clause a: determinates {1,2,3,4,5,6}, {2,4,6} for true and {1,3,5} for false
    // for major clause b: determinates {5,7}, {7} for true and {5} for false
    // for major clause c: determinates {3,7}, {7} for true and {3} for false
    // test requirements: {2,3,5,7}, therefore:
    //  a + !b*!c, !a + b*!c, !a + !b*c, !a + b*c

    // General Inactive Clause Coverage (GICC) adds the following tests:
    // for major clause a: non-determinates {7,8}, adding {8} for true, no feasible requirement for false predicate
    // for major clause b: non-determinates {1,2,3,4,6,8}
    //   {8} for true clause and true predicate
    //   {4,6} for false clause and true predicate
    //   {3} for true clause and false predicate
    //   {1} for false clause and false predicate
    //   adding {1,4}
    // for major clause c: non-determinates {1,2,4,5,6,8}
    //   {6,8} for true clause and true predicate
    //   {2,4} for false clause and true predicate
    //   {5} for true clause and false predicate
    //   {1} for false clause and false predicate
    //   adding none
    // test requirements of CACC and GICC: {1,2,3,4,5,7,8}, therefore:
    //  !a + !b*!c, a + !b*!c, !a + b*!c, a + b*!c, !a + !b*c, !a + b*c, a + b*c

    let result_not_a_not_b_not_c = try_parse_dispel(
        &UnAura {
            un_aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            target: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            un_aura_spell_id: 42,
            target_spell_id: 42,
            // right part of a:
            un_aura_amount: 1,
        },
        // amount of AuraApplications is left part of a and complete b:
        &[Event {
            id: 42,
            timestamp: 1337,
            subject: DomainValueUnit::Creature(Creature {
                creature_id: 0xF130 << 48,
                entry: 0,
                owner: Some(0xF130 << 48),
            }),
            event: EventType::SpellCast(SpellCast {
                victim: Some(DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                })),
                hit_type: HitType::Evade,
                spell_id: Some(42),
                damage: vec![],
                heal: vec![],
                threat: vec![],
            }),
        }],
        // left part of c:
        1337,
        // left part of c:
        1337,
        &armory,
        42,
        &summons,
    );
    assert!(result_not_a_not_b_not_c.is_err());

    let result_a_not_b_not_c = try_parse_dispel(
        &UnAura {
            un_aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            target: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            un_aura_spell_id: 42,
            target_spell_id: 42,
            // right part of a:
            un_aura_amount: 0,
        },
        // amount of AuraApplications is left part of a and complete b:
        &[Event {
            id: 42,
            timestamp: 1337,
            subject: DomainValueUnit::Creature(Creature {
                creature_id: 0xF130 << 48,
                entry: 0,
                owner: Some(0xF130 << 48),
            }),
            event: EventType::SpellCast(SpellCast {
                victim: Some(DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                })),
                hit_type: HitType::Evade,
                spell_id: Some(42),
                damage: vec![],
                heal: vec![],
                threat: vec![],
            }),
        }],
        // left part of c:
        1337,
        // left part of c:
        1337,
        &armory,
        42,
        &summons,
    );
    assert!(result_a_not_b_not_c.is_ok());
    let result_a_not_b_not_c = result_a_not_b_not_c.unwrap();
    assert_eq!(result_a_not_b_not_c.0, 42);
    assert_eq!(result_a_not_b_not_c.1.len(), 0);

    let result_not_a_b_not_c = try_parse_dispel(
        &UnAura {
            un_aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            target: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            un_aura_spell_id: 42,
            target_spell_id: 42,
            // right part of a:
            un_aura_amount: 2,
        },
        // amount of AuraApplications is left part of a and complete b:
        &[
            Event {
                id: 42,
                timestamp: 1337,
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::AuraApplication(AuraApplication {
                    caster: DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    }),
                    stack_amount: 42,
                    spell_id: 42,
                }),
            },
            Event {
                id: 42,
                timestamp: 1337,
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::SpellCast(SpellCast {
                    victim: Some(DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    })),
                    hit_type: HitType::Evade,
                    spell_id: Some(42),
                    damage: vec![],
                    heal: vec![],
                    threat: vec![],
                }),
            },
        ],
        // left part of c:
        1337,
        // left part of c:
        1337,
        &armory,
        42,
        &summons,
    );
    assert!(result_not_a_b_not_c.is_err());

    let result_a_b_not_c = try_parse_dispel(
        &UnAura {
            un_aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            target: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            un_aura_spell_id: 42,
            target_spell_id: 42,
            // right part of a:
            un_aura_amount: 1,
        },
        // amount of AuraApplications is left part of a and complete b:
        &[
            Event {
                id: 42,
                timestamp: 1337,
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::AuraApplication(AuraApplication {
                    caster: DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    }),
                    stack_amount: 42,
                    spell_id: 42,
                }),
            },
            Event {
                id: 42,
                timestamp: 1337,
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::SpellCast(SpellCast {
                    victim: Some(DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    })),
                    hit_type: HitType::Evade,
                    spell_id: Some(42),
                    damage: vec![],
                    heal: vec![],
                    threat: vec![],
                }),
            },
        ],
        // left part of c:
        1337,
        // left part of c:
        1337,
        &armory,
        42,
        &summons,
    );
    assert!(result_a_b_not_c.is_ok());
    let result_a_b_not_c = result_a_b_not_c.unwrap();
    assert_eq!(result_a_b_not_c.0, 42);
    assert_eq!(result_a_b_not_c.1.len(), 1);
    assert_eq!(result_a_b_not_c.1[0], 42);

    let result_not_a_not_b_c = try_parse_dispel(
        &UnAura {
            un_aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            target: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            un_aura_spell_id: 42,
            target_spell_id: 42,
            // right part of a:
            un_aura_amount: 1,
        },
        // amount of AuraApplications is left part of a and complete b:
        &[Event {
            id: 42,
            timestamp: 1337,
            subject: DomainValueUnit::Creature(Creature {
                creature_id: 0xF130 << 48,
                entry: 0,
                owner: Some(0xF130 << 48),
            }),
            event: EventType::SpellCast(SpellCast {
                victim: Some(DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                })),
                hit_type: HitType::Evade,
                spell_id: Some(42),
                damage: vec![],
                heal: vec![],
                threat: vec![],
            }),
        }],
        // left part of c:
        1337,
        // left part of c:
        1348,
        &armory,
        42,
        &summons,
    );
    assert!(result_not_a_not_b_c.is_err());

    let result_a_not_b_c = try_parse_dispel(
        &UnAura {
            un_aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            target: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            un_aura_spell_id: 42,
            target_spell_id: 42,
            // right part of a:
            un_aura_amount: 0,
        },
        // amount of AuraApplications is left part of a and complete b:
        &[Event {
            id: 42,
            timestamp: 1337,
            subject: DomainValueUnit::Creature(Creature {
                creature_id: 0xF130 << 48,
                entry: 0,
                owner: Some(0xF130 << 48),
            }),
            event: EventType::SpellCast(SpellCast {
                victim: Some(DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                })),
                hit_type: HitType::Evade,
                spell_id: Some(42),
                damage: vec![],
                heal: vec![],
                threat: vec![],
            }),
        }],
        // left part of c:
        1337,
        // left part of c:
        1348,
        &armory,
        42,
        &summons,
    );
    assert!(result_a_not_b_c.is_ok());
    let result_a_not_b_c = result_a_not_b_c.unwrap();
    assert_eq!(result_a_not_b_c.0, 42);
    assert_eq!(result_a_not_b_c.1.len(), 0);

    let result_not_a_b_c = try_parse_dispel(
        &UnAura {
            un_aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            target: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            un_aura_spell_id: 42,
            target_spell_id: 42,
            // right part of a:
            un_aura_amount: 2,
        },
        // amount of AuraApplications is left part of a and complete b:
        &[
            Event {
                id: 42,
                timestamp: 1337,
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::AuraApplication(AuraApplication {
                    caster: DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    }),
                    stack_amount: 42,
                    spell_id: 42,
                }),
            },
            Event {
                id: 42,
                timestamp: 1337,
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::SpellCast(SpellCast {
                    victim: Some(DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    })),
                    hit_type: HitType::Evade,
                    spell_id: Some(42),
                    damage: vec![],
                    heal: vec![],
                    threat: vec![],
                }),
            },
        ],
        // left part of c:
        1337,
        // left part of c:
        1348,
        &armory,
        42,
        &summons,
    );
    assert!(result_not_a_b_c.is_ok());
    let result_not_a_b_c = result_not_a_b_c.unwrap();
    assert_eq!(result_not_a_b_c.0, 42);
    assert_eq!(result_not_a_b_c.1.len(), 1);
    assert_eq!(result_not_a_b_c.1[0], 42);

    let result_a_b_c = try_parse_dispel(
        &UnAura {
            un_aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            target: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            un_aura_spell_id: 42,
            target_spell_id: 42,
            // right part of a:
            un_aura_amount: 1,
        },
        // amount of AuraApplications is left part of a and complete b:
        &[
            Event {
                id: 42,
                timestamp: 1337,
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::AuraApplication(AuraApplication {
                    caster: DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    }),
                    stack_amount: 42,
                    spell_id: 42,
                }),
            },
            Event {
                id: 42,
                timestamp: 1337,
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::SpellCast(SpellCast {
                    victim: Some(DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    })),
                    hit_type: HitType::Evade,
                    spell_id: Some(42),
                    damage: vec![],
                    heal: vec![],
                    threat: vec![],
                }),
            },
        ],
        // left part of c:
        1337,
        // left part of c:
        1348,
        &armory,
        42,
        &summons,
    );
    assert!(result_a_b_c.is_ok());
    let result_a_b_c = result_a_b_c.unwrap();
    assert_eq!(result_a_b_c.0, 42);
    assert_eq!(result_a_b_c.1.len(), 1);
    assert_eq!(result_a_b_c.1[0], 42);
}
