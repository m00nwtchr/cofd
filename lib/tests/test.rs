use cofd::{
	ability::Ability,
	merits::Merit,
	prelude::{Attribute::*, Skill::*, werewolf::*, *},
	schema::template::SupernaturalTolerance,
	splat::{SplatCharacter, werewolf::MoonGift},
};
use cofd_schema::traits::DerivedTrait::Health;

#[test]
fn size() {
	println!("{}", size_of::<SplatCharacter>());
}

#[test]
fn it_works() {
	let mut character = Character::builder()
		.with_splat(
			Werewolf::new()
				.with_auspice(Auspice::Rahu)
				.with_tribe(ForsakenTribe::BloodTalons),
		)
		// .with_info(CharacterInfo {
		// 	name: String::from("Amos Gray"),
		// 	player: String::from("m00n"),
		// 	virtue_anchor: String::from("Destroyer"),
		// 	vice_anchor: String::from("Lone Wolf"),
		// 	..Default::default()
		// })
		.with_attributes(vec![
			(Intelligence, 1),
			(Wits, 3),
			(Resolve, 2),
			(Strength, 3),
			(Dexterity, 2),
			(Stamina, 3),
			(Presence, 3),
			(Manipulation, 1),
			(Composure, 3),
		])
		.with_skills(vec![
			(Investigation, 2),
			(Medicine, 2),
			(Athletics, 2),
			(Brawl, 4),
			(Stealth, 2),
			(Survival, 3),
			(Expression, 3),
			(Intimidation, 4),
		])
		.with_specialties(Brawl, vec![String::from("Claws")])
		.with_specialties(Stealth, vec![String::from("Stalking")])
		.with_specialties(Intimidation, vec![String::from("Direct Threats")])
		.with_abilities([(Renown::Glory.into(), 1), (Renown::Purity.into(), 3)])
		.with_merits([
			(Merit::Giant, 3),
			(Merit::TrainedObserver, 1),
			(Merit::DefensiveCombat(true, Brawl), 1),
			(WerewolfMerit::FavoredForm { form: Form::Gauru }.into(), 2),
			(WerewolfMerit::EfficientKiller.into(), 2),
			(Merit::RelentlessAssault, 2),
			(Merit::Language("First Tongue".to_owned()), 1),
			(WerewolfMerit::Totem.into(), 1),
		])
		.build();

	assert_eq!(
		Some(1),
		character.attributes().value(&Trait::SupernaturalTolerance(
			SupernaturalTolerance::PrimalUrge,
		))
	);
	assert_eq!(
		Some(3),
		character
			.attributes()
			.value(&Trait::Ability(Ability::MoonGift(MoonGift::Full)))
	);

	assert_eq!(
		Some(3),
		character.attributes().value(&Trait::Attribute(Stamina))
	);
	assert_eq!(
		Some(11),
		character.attributes().value(&Trait::DerivedTrait(Health))
	);

	character
		.attributes_mut()
		.set_raw_value(&Trait::Ability(Ability::Renown(Renown::Purity)), 1);

	assert_eq!(
		Some(8),
		character.attributes().value(&Trait::DerivedTrait(Health))
	);

	character.set_form(Form::Gauru);

	assert_eq!(
		Some(5),
		character.attributes().value(&Trait::Attribute(Stamina))
	);
	assert_eq!(
		Some(12),
		character.attributes().value(&Trait::DerivedTrait(Health))
	);

	character
		.attributes_mut()
		.set_raw_value(&Trait::Ability(Ability::Renown(Renown::Purity)), 3);

	assert_eq!(
		Some(15),
		character.attributes().value(&Trait::DerivedTrait(Health))
	);

	let character = SplatCharacter::Werewolf(character);

	assert_eq!(
		Some(5),
		character.attributes().value(&Trait::Attribute(Stamina))
	);
}

//
// #[test]
// #[allow(clippy::too_many_lines)]
// fn it_works() {
// 	let vampire_character = Character::builder()
// 		.with_splat(
// 			Vampire::new(
// 				Clan::Ventrue,
// 				Some(Covenant::OrdoDracul),
// 				Some(Bloodline::Custom(
// 					"Dragolescu".to_string(),
// 					Some([
// 						Discipline::Animalism,
// 						Discipline::Dominate,
// 						Discipline::Resilience,
// 						Discipline::Auspex,
// 					]),
// 				)),
// 			)
// 			.with_attr_bonus(Attribute::Resolve),
// 		)
// 		.with_info(CharacterInfo {
// 			name: String::from("Darren Webb"),
// 			player: String::from("m00n"),
// 			chronicle: String::from("Night Trains"),
// 			virtue_anchor: String::from("Scholar"),
// 			vice_anchor: String::from("Authoritarian"),
// 			concept: String::from("Occult Journalist/Mastermind"),
// 			..Default::default()
// 		})
// 		.with_attributes(Attributes {
// 			intelligence: 3,
// 			wits: 3,
// 			resolve: 2,
// 			strength: 1,
// 			dexterity: 3,
// 			stamina: 2,
// 			presence: 3,
// 			manipulation: 2,
// 			composure: 3,
// 		})
// 		.with_skills(Skills {
// 			investigation: 2,
// 			occult: 3,
// 			politics: 2,
// 			larceny: 3,
// 			stealth: 1,
// 			animal_ken: 1,
// 			expression: 3,
// 			intimidation: 1,
// 			streetwise: 2,
// 			subterfuge: 4,
// 			..Default::default()
// 		})
// 		.with_specialties(Skill::Larceny, vec![String::from("Sleight of Hand")])
// 		.with_specialties(Skill::Streetwise, vec![String::from("Rumours")])
// 		.with_specialties(Skill::Subterfuge, vec![String::from("Detecting Lies")])
// 		.with_abilities([
// 			(Discipline::Animalism.into(), 1),
// 			(Discipline::Dominate.into(), 2),
// 			(
// 				Discipline::Custom("Coil of the Voivode".to_string()).into(),
// 				2,
// 			),
// 		])
// 		.with_merits([
// 			(Merit::Status("Ordo Dracul".to_string()), 1),
// 			(Merit::Status("City".to_string()), 1),
// 			(VampireMerit::CacophonySavvy.into(), 3),
// 			(Merit::FastTalking, 1),
// 			(
// 				Merit::ProfessionalTraining {
// 					profession: String::new(),
// 					skills: [Skill::Expression, Skill::Occult],
// 					skill: None,
// 				},
// 				2,
// 			),
// 			// AbilityVal(Ability::Merit(Merit::Contacts(String::new())), 2),
// 			(Merit::SafePlace(String::new()), 3),
// 			(Merit::Resources, 3),
// 			(VampireMerit::NestGuardian.into(), 1),
// 		])
// 		.build();
//
// 	// vampire_character.splat.vice_anchor();
//
// 	println!("{vampire_character:?}");
// 	// println!("{:?}", vampire_character.attributes());
//
// 	println!(
// 		"{}",
// 		ron::ser::to_string_pretty(&vampire_character, PrettyConfig::default()).unwrap()
// 	);
//
// 	// assert_eq!(vampire_character.max_health(), 7);
// 	// assert_eq!(vampire_character.attributes().strength, 1);
// 	// assert_eq!(vampire_character.max_fuel(), 10);
//
// 	let mut werewolf_character = Character::builder()
// 		.with_splat(
// 			Werewolf::new()
// 				.with_auspice(Auspice::Rahu)
// 				.with_tribe(Tribe::BloodTalons),
// 		)
// 		.with_info(CharacterInfo {
// 			name: String::from("Amos Gray"),
// 			player: String::from("m00n"),
// 			virtue_anchor: String::from("Destroyer"),
// 			vice_anchor: String::from("Lone Wolf"),
// 			..Default::default()
// 		})
// 		.with_attributes(Attributes {
// 			intelligence: 1,
// 			wits: 3,
// 			resolve: 2,
// 			strength: 3,
// 			dexterity: 2,
// 			stamina: 3,
// 			presence: 3,
// 			manipulation: 1,
// 			composure: 3,
// 		})
// 		.with_skills(Skills {
// 			investigation: 2,
// 			medicine: 2,
// 			athletics: 2,
// 			brawl: 4,
// 			stealth: 2,
// 			survival: 3,
// 			expression: 3,
// 			intimidation: 4,
// 			..Default::default()
// 		})
// 		.with_specialties(Skill::Brawl, vec![String::from("Claws")])
// 		.with_specialties(Skill::Stealth, vec![String::from("Stalking")])
// 		.with_specialties(Skill::Intimidation, vec![String::from("Direct Threats")])
// 		.with_abilities([(Renown::Glory.into(), 1), (Renown::Purity.into(), 3)])
// 		.with_merits([
// 			(Merit::Giant, 3),
// 			(Merit::TrainedObserver, 1),
// 			(Merit::DefensiveCombat(true, Some(Skill::Brawl)), 1),
// 			(WerewolfMerit::FavoredForm { form: Form::Gauru }.into(), 2),
// 			(WerewolfMerit::EfficientKiller.into(), 2),
// 			(Merit::RelentlessAssault, 2),
// 			(Merit::Language("First Tongue".to_owned()), 1),
// 			(WerewolfMerit::Totem.into(), 1),
// 		])
// 		.build();
//
// 	werewolf_character.power = 3;
//
// 	println!("{werewolf_character:?}");
//
// 	// assert_eq!(werewolf_character.max_fuel(), 12);
// 	// assert_eq!(werewolf_character.defense(), 6);
// 	// assert_eq!(werewolf_character.perception(), 7);
// 	// assert_eq!(werewolf_character.max_health(), 12);
//
// 	if let Splat::Werewolf(.., ww) = &mut werewolf_character.splat {
// 		ww.form = Form::Gauru;
// 	}
//
// 	// assert_eq!(werewolf_character.perception(), 7);
//
// 	let t = std::time::Instant::now();
// 	// werewolf_character.calc_mod_map();
// 	println!("{:?}", std::time::Instant::now().duration_since(t));
//
// 	// assert_eq!(werewolf_character.perception(), 9);
//
// 	let mut mage_character = Character::builder()
// 		.with_splat(Mage::new(Path::Mastigos).with_order(Order::Mysterium))
// 		.with_info(CharacterInfo {
// 			name: String::from("Polaris"),
// 			player: String::from("m00n"),
// 			virtue_anchor: String::from("Curious"),
// 			vice_anchor: String::from("Greedy"),
// 			concept: String::from("Astronomer"),
// 			..Default::default()
// 		})
// 		.with_attributes(Attributes {
// 			intelligence: 3,
// 			wits: 3,
// 			resolve: 5,
// 			strength: 2,
// 			dexterity: 3,
// 			stamina: 2,
// 			presence: 1,
// 			manipulation: 2,
// 			composure: 3,
// 		})
// 		.with_skills(Skills {
// 			academics: 2,
// 			computer: 1,
// 			crafts: 1,
// 			investigation: 3,
// 			occult: 3,
// 			science: 2,
//
// 			larceny: 2,
// 			stealth: 2,
//
// 			animal_ken: 1,
// 			empathy: 2,
// 			expression: 1,
// 			subterfuge: 3,
// 			..Default::default()
// 		})
// 		.with_specialties(Skill::Academics, vec![String::from("Research")])
// 		.with_specialties(Skill::AnimalKen, vec![String::from("Felines")])
// 		.with_specialties(Skill::Subterfuge, vec![String::from("Detecting Lies")])
// 		// TODO: Professional Training specialties
// 		.with_specialties(Skill::Investigation, vec![String::from("Riddles")])
// 		.with_specialties(Skill::Science, vec![String::from("Astronomy")])
// 		.with_abilities([
// 			(Arcanum::Mind.into(), 1),
// 			(Arcanum::Prime.into(), 2),
// 			(Arcanum::Space.into(), 3),
// 		])
// 		.with_merits([
// 			(Merit::Status("Mysterium".to_string()), 1),
// 			(MageMerit::HighSpeech.into(), 1),
// 			(
// 				Merit::ProfessionalTraining {
// 					profession: "e".to_owned(),
// 					skills: [Skill::Investigation, Skill::Science],
// 					skill: None,
// 				},
// 				3,
// 			),
// 			(Merit::TrainedObserver, 1),
// 			//
// 			//
// 		])
// 		.build();
//
// 	// mage_character.calc_mod_map();
//
// 	if let Splat::Mage(.., data) = &mut mage_character.splat {
// 		data.set_attr_bonus(Attribute::Resolve);
// 	}
//
// 	// mage_character.calc_mod_map();
// 	//
// 	// assert_ne!(mage_character.attributes().resolve, 6);
// 	//
// 	// mage_character.base_attributes_mut().resolve = 4;
// 	// assert_eq!(mage_character.attributes().resolve, 5);
// }
