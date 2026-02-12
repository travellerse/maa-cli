use anyhow::bail;
use clap::ValueEnum;
use maa_value::{MAAValue, insert, object};

#[repr(i8)]
#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Clone, Copy)]
pub enum Theme {
    Phantom,
    Mizuki,
    Sami,
    Sarkaz,
    JieGarden,
}

impl Theme {
    const fn to_str(self) -> &'static str {
        match self {
            Self::Phantom => "Phantom",
            Self::Mizuki => "Mizuki",
            Self::Sami => "Sami",
            Self::Sarkaz => "Sarkaz",
            Self::JieGarden => "JieGarden",
        }
    }
}

impl ValueEnum for Theme {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Phantom,
            Self::Mizuki,
            Self::Sami,
            Self::Sarkaz,
            Self::JieGarden,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.to_str()))
    }
}

#[derive(clap::Args)]
pub struct RoguelikeParams {
    /// Theme of the roguelike
    theme: Theme,
    /// Mode of the roguelike
    ///
    /// 0: 刷经验 (exp mode)
    /// 1: 刷源石锭 (investment mode, complete first floor investment then exit)
    /// 2: (Deprecated) Combined mode 0 and 1
    /// 3: (Not implemented) Ending mode
    /// 4: 刷开局 (collectible mode, farm for specific start rewards)
    /// 5: 刷坍缩范式 (Collapsal Paradigms mode, Sami only)
    /// 6: 月度小队 (monthly squad mode)
    /// 7: 深入调查 (deep exploration mode)
    /// 20001: 刷常乐节点 (find playtime mode, JieGarden only)
    #[arg(long, default_value = "0")]
    mode: i32,

    // TODO: input localized names, maybe during initialization of tasks

    /// Starting squad (Chinese name)
    #[arg(long)]
    squad: Option<String>,
    /// Core operator (Chinese name)
    #[arg(long)]
    core_char: Option<String>,
    /// Starting recruitment combination (Chinese name)
    #[arg(long)]
    roles: Option<String>,

    /// Stop after given count, if not given, never stop
    #[arg(long)]
    start_count: Option<i32>,

    /// Difficulty, not valid for Phantom theme
    #[arg(long)]
    difficulty: Option<i32>,

    /// Disable investment
    #[arg(long)]
    disable_investment: bool,
    /// Invest with more score (招募、购物刷分)
    #[arg(long)]
    investment_with_more_score: bool,
    /// Stop after given investment count
    #[arg(long)]
    investments_count: Option<i32>,
    /// Do not stop when investment is full
    #[arg(long)]
    no_stop_when_investment_full: bool,

    /// Use support operator
    #[arg(long)]
    use_support: bool,
    /// Use non-friend support
    #[arg(long)]
    use_nonfriend_support: bool,

    /// Start with elite two (凹开局干员精二直升)
    #[arg(long)]
    start_with_elite_two: bool,
    /// Only start with elite two, do not fight (只凹精二直升且不进行作战)
    #[arg(long)]
    only_start_with_elite_two: bool,

    /// Stop before final boss
    #[arg(long)]
    stop_at_final_boss: bool,
    /// Stop when deposit is full
    #[arg(long)]
    stop_when_deposit_full: bool,
    /// Stop when level reaches max
    #[arg(long)]
    stop_when_level_max: bool,

    /// Refresh trader with dice (Mizuki theme only)
    #[arg(long)]
    refresh_trader_with_dice: bool,

    /// Enable foldartal system (远见, Sami theme)
    #[arg(long)]
    use_foldartal: bool,
    /// Starting foldartals (Chinese names, can be used multiple times)
    #[arg(short = 'F', long)]
    start_foldartals: Vec<String>,
    /// Expected collapsal paradigms for mode 5 (Chinese names, required for mode 5)
    #[arg(short = 'P', long)]
    expected_collapsal_paradigms: Vec<String>,
    /// First floor foldartal collection in mode 4 (Sami theme)
    #[arg(long)]
    sami_first_floor_foldartal: bool,
    /// First floor foldartal list (Chinese names)
    #[arg(long)]
    sami_first_floor_foldartals: Vec<String>,
    /// Enable starting foldartal for 生活至上分队 (Sami theme)
    #[arg(long)]
    sami_new_squad2_starting_foldartal: bool,
    /// Starting foldartal for 生活至上分队 (takes precedence over general start_foldartals)
    #[arg(long)]
    sami_new_squad2_starting_foldartals: Vec<String>,

    /// Start with seed (Sarkaz theme mode 1 only, use with --seed)
    #[arg(long)]
    start_with_seed: bool,
    /// Seed value for deterministic runs
    #[arg(long)]
    seed: Option<String>,

    /// Target bosky node for mode 20001 (1=令(掷地有声), 2=黍(种因得果), 3=年(三缺一))
    #[arg(long)]
    find_playtime_target: Option<i32>,

    /// Squad for collectible mode (different from main squad, for 烧水 strategy)
    #[arg(long)]
    collectible_squad: Option<String>,
    /// Enable shopping in collectible mode
    #[arg(long)]
    collectible_shopping: bool,
    /// Start rewards comma-separated list: hot_water,shield,ingot,hope,random,key,dice,idea,ticket
    #[arg(long)]
    collectible_start_awards: Option<String>,

    /// Auto iterate through monthly squads (mode 6)
    #[arg(long, default_value = "true")]
    monthly_squad_auto_iterate: bool,
    /// Check and collect communication items (mode 6)
    #[arg(long, default_value = "true")]
    monthly_squad_check_comms: bool,

    /// Auto iterate through deep exploration missions (mode 7)
    #[arg(long, default_value = "true")]
    deep_exploration_auto_iterate: bool,
}

impl super::ToTaskType for RoguelikeParams {
    fn to_task_type(&self) -> super::TaskType {
        super::TaskType::Roguelike
    }
}

impl super::IntoParameters for RoguelikeParams {
    fn into_parameters_no_context(self) -> anyhow::Result<MAAValue> {
        let theme = self.theme;
        let mode = self.mode;

        match mode {
            5 if !matches!(theme, Theme::Sami) => {
                bail!("Mode 5 is only available in Sami theme");
            }
            20001 if !matches!(theme, Theme::JieGarden) => {
                bail!("Mode 20001 is only available in JieGarden theme");
            }
            0..=7 | 20001 => {} // Allow modes 0-7 and 20001
            _ => bail!("Mode must be in range between 0 and 7, or 20001"),
        }

        // Validate seed parameters
        if self.start_with_seed && (!matches!(theme, Theme::Sarkaz) || mode != 1) {
            log::warn!("start_with_seed is only meaningful for Sarkaz theme with mode 1, ignoring");
        }
        if self.seed.is_some() && !self.start_with_seed {
            log::warn!("seed is provided but start_with_seed is not enabled, ignoring");
        }

        // Validate collectible mode parameters
        if mode != 4 {
            if self.collectible_squad.is_some() || self.collectible_shopping || self.collectible_start_awards.is_some() {
                log::warn!("Collectible mode parameters are only meaningful for mode 4, ignoring");
            }
        }

        // Validate monthly squad parameters  
        if mode != 6 && (self.monthly_squad_auto_iterate || self.monthly_squad_check_comms) {
            log::warn!("Monthly squad parameters are only meaningful for mode 6, ignoring");
        }

        // Validate deep exploration parameters
        if mode != 7 && self.deep_exploration_auto_iterate {
            log::warn!("Deep exploration parameters are only meaningful for mode 7, ignoring");
        }

        let mut value = object!(
            "theme" => self.theme.to_str(),
            "mode" => self.mode,
            "squad" =>? self.squad,
            "roles" =>? self.roles,
            "core_char" =>? self.core_char,
            "start_count" =>? self.start_count,
            "stop_at_final_boss" => self.stop_at_final_boss,
            "stop_when_deposit_full" => self.stop_when_deposit_full,
            "stop_at_max_level" => self.stop_when_level_max,
        );

        // Difficulty setting (not valid for Phantom theme)
        if matches!(theme, Theme::Phantom) {
            if self.difficulty.is_some() {
                log::warn!("Difficulty is not valid for Phantom theme, ignored");
            }
        } else {
            insert!(value, "difficulty" =>? self.difficulty);
        }

        // Investment mode settings
        if self.disable_investment {
            value.insert("investment_enabled", false.into());
        } else {
            insert!(value,
                "investment_enabled" => true,
                "investments_count" =>? self.investments_count,
                "investment_with_more_score" => self.investment_with_more_score,
                "stop_when_investment_full" => !self.no_stop_when_investment_full,
            );
        }

        // Support unit settings
        insert!(value,
            "use_support" => self.use_support,
            "use_nonfriend_support" => self.use_nonfriend_support
        );

        // Elite settings
        if self.start_with_elite_two {
            insert!(value,
                "start_with_elite_two" => true,
                "only_start_with_elite_two" => self.only_start_with_elite_two
            );
        }

        // Collectible mode settings
        if mode == 4 { // Collectible mode
            insert!(value,
                "collectible_mode_squad" =>? self.collectible_squad,
                "collectible_mode_shopping" => self.collectible_shopping,
            );

            if let Some(awards) = &self.collectible_start_awards {
                let reward_map = [
                    ("hot_water", "hot_water"),
                    ("shield", "shield"),
                    ("ingot", "ingot"),
                    ("hope", "hope"),
                    ("random", "random"),
                    ("key", "key"),
                    ("dice", "dice"),
                    ("idea", "ideas"),
                    ("ticket", "ticket"),
                ];

                let mut start_rewards = object!();
                let mut valid_count = 0;
                for award in awards.split(',') {
                    let award = award.trim();
                    if let Some((_, key)) = reward_map.iter().find(|(name, _)| *name == award) {
                        start_rewards.insert(key.to_string(), true.into());
                        valid_count += 1;
                    } else if !award.is_empty() {
                        log::warn!("Unknown collectible start award: '{}', ignoring", award);
                    }
                }
                if valid_count > 0 {
                    value.insert("collectible_mode_start_list", start_rewards);
                }
            }
        }

        // Monthly squad mode settings
        if mode == 6 { // Squad mode
            insert!(value,
                "monthly_squad_auto_iterate" => self.monthly_squad_auto_iterate,
                "monthly_squad_check_comms" => self.monthly_squad_check_comms,
            );
        }

        // Deep exploration mode settings
        if mode == 7 { // Exploration mode
            value.insert("deep_exploration_auto_iterate", self.deep_exploration_auto_iterate.into());
        }

        // Theme specific parameters
        match theme {
            Theme::Mizuki => {
                value.insert(
                    "refresh_trader_with_dice",
                    self.refresh_trader_with_dice.into(),
                );
            }
            Theme::Sami => {
                value.insert("use_foldartal", self.use_foldartal.into());

                // First floor foldartal collection in collectible mode
                if mode == 4 && self.sami_first_floor_foldartal && !self.sami_first_floor_foldartals.is_empty() {
                    insert!(value, "first_floor_foldartal" => self.sami_first_floor_foldartals?);
                }

                // Starting foldartal for life squad (takes precedence over general start_foldartals)
                if self.sami_new_squad2_starting_foldartal && !self.sami_new_squad2_starting_foldartals.is_empty() {
                    insert!(value, "start_foldartal_list" => self.sami_new_squad2_starting_foldartals?);
                } else if !self.start_foldartals.is_empty() {
                    insert!(value, "start_foldartal_list" => self.start_foldartals?);
                }

                if mode == 5 {
                    if self.expected_collapsal_paradigms.is_empty() {
                        bail!(
                            "At least one expected collapsal paradigm is required when mode 5 is enabled"
                        );
                    }
                    insert!(value,
                        "check_collapsal_paradigms" => true,
                        "double_check_collapsal_paradigms" => true,
                        "expected_collapsal_paradigms" => self.expected_collapsal_paradigms?,

                    );
                }
            }
            Theme::Sarkaz if mode == 1 => {
                // Only set start_with_seed if both flag is true and seed is provided
                if self.start_with_seed {
                    if let Some(seed) = &self.seed {
                        value.insert("start_with_seed", seed.clone().into());
                    } else {
                        bail!("Seed must be provided when start_with_seed is enabled");
                    }
                }
            }
            Theme::JieGarden if mode == 20001 => {
                if let Some(target) = self.find_playtime_target {
                    if !(1..=3).contains(&target) {
                        bail!("find_playtime_target must be between 1 and 3");
                    }
                    insert!(value, "find_playTime_target" => target);
                } else {
                    bail!("find_playtime_target is required for JieGarden theme with mode 20001");
                }
            }
            _ => {}
        }

        Ok(value)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use maa_value::object;

    use super::*;
    use crate::command::{Command, parse_from};

    mod theme {
        use super::*;

        #[test]
        fn to_str() {
            assert_eq!(Theme::Phantom.to_str(), "Phantom");
            assert_eq!(Theme::Mizuki.to_str(), "Mizuki");
            assert_eq!(Theme::Sami.to_str(), "Sami");
            assert_eq!(Theme::Sarkaz.to_str(), "Sarkaz");
            assert_eq!(Theme::JieGarden.to_str(), "JieGarden");
        }

        #[test]
        fn value_variants() {
            assert_eq!(Theme::value_variants(), &[
                Theme::Phantom,
                Theme::Mizuki,
                Theme::Sami,
                Theme::Sarkaz,
                Theme::JieGarden,
            ]);
        }

        #[test]
        fn to_possible_value() {
            assert_eq!(
                Theme::Phantom.to_possible_value(),
                Some(clap::builder::PossibleValue::new("Phantom"))
            );
            assert_eq!(
                Theme::Mizuki.to_possible_value(),
                Some(clap::builder::PossibleValue::new("Mizuki"))
            );
            assert_eq!(
                Theme::Sami.to_possible_value(),
                Some(clap::builder::PossibleValue::new("Sami"))
            );
            assert_eq!(
                Theme::Sarkaz.to_possible_value(),
                Some(clap::builder::PossibleValue::new("Sarkaz"))
            );
            assert_eq!(
                Theme::JieGarden.to_possible_value(),
                Some(clap::builder::PossibleValue::new("JieGarden"))
            );
        }
    }

    #[test]
    fn parse_roguellike_params() {
        fn parse<I, T>(args: I) -> Result<MAAValue, anyhow::Error>
        where
            I: IntoIterator<Item = T>,
            T: Into<std::ffi::OsString> + Clone,
        {
            let command = parse_from(args).command;
            match command {
                Command::Roguelike { params, .. } => {
                    use super::super::{IntoParameters, TaskType, ToTaskType};
                    assert_eq!(params.to_task_type(), TaskType::Roguelike);
                    params.into_parameters_no_context()
                }
                _ => panic!("Not a Roguelike command"),
            }
        }

        let default_params = object!(
            "mode" => 0,
            "investment_enabled" => true,
            "investment_with_more_score" => false,
            "stop_when_investment_full" => true,
            "stop_at_final_boss" => false,
            "stop_when_deposit_full" => false,
            "stop_at_max_level" => false,
            "use_support" => false,
            "use_nonfriend_support" => false,
        );

        assert_eq!(
            parse(["maa", "roguelike", "Phantom"]).unwrap(),
            default_params.join(object!("theme" => "Phantom")),
        );
        // Mode 5 is only available for Sami theme
        assert!(parse(["maa", "roguelike", "Phantom", "--mode", "5"]).is_err());

        // Difficulty is ignored for Phantom theme
        assert_eq!(
            parse(["maa", "roguelike", "Phantom", "--difficulty", "15"]).unwrap(),
            default_params.join(object!("theme" => "Phantom")),
        );

        assert_eq!(
            parse([
                "maa",
                "roguelike",
                "Sarkaz",
                "--squad",
                "蓝图测绘分队",
                "--roles",
                "取长补短",
                "--core-char",
                "维什戴尔",
                "--start-count=100",
                "--difficulty=15",
            ])
            .unwrap(),
            default_params.join(object!(
                "theme" => "Sarkaz",
                "squad" => "蓝图测绘分队",
                "roles" => "取长补短",
                "core_char" => "维什戴尔",
                "start_count" => 100,
                "difficulty" => 15,
            )),
        );

        assert_eq!(
            parse(["maa", "roguelike", "Sarkaz", "--disable-investment"]).unwrap(),
            // Can't use default_params here because some fields are removed in this case
            object!(
                "theme" => "Sarkaz",
                "mode" => 0,
                "investment_enabled" => false,
                "stop_at_final_boss" => false,
                "stop_when_deposit_full" => false,
                "stop_at_max_level" => false,
                "use_support" => false,
                "use_nonfriend_support" => false,
            ),
        );
        assert_eq!(
            parse([
                "maa",
                "roguelike",
                "Sarkaz",
                "--investment-with-more-score",
                "--investments-count=100",
                "--no-stop-when-investment-full"
            ])
            .unwrap(),
            default_params.join(object!(
                "theme" => "Sarkaz",
                "investment_with_more_score" => true,
                "investments_count" => 100,
                "stop_when_investment_full" => false,
            )),
        );

        assert_eq!(
            parse([
                "maa",
                "roguelike",
                "Sarkaz",
                "--use-support",
                "--use-nonfriend-support",
                "--start-with-elite-two",
                "--only-start-with-elite-two",
                "--stop-at-final-boss",
            ])
            .unwrap(),
            default_params.join(object!(
                "theme" => "Sarkaz",
                "use_support" => true,
                "use_nonfriend_support" => true,
                "start_with_elite_two" => true,
                "only_start_with_elite_two" => true,
                "stop_at_final_boss" => true,
            )),
        );

        assert_eq!(
            parse(["maa", "roguelike", "Mizuki"]).unwrap(),
            default_params.join(object!(
                "theme" => "Mizuki",
                "refresh_trader_with_dice" => false,
            )),
        );

        assert_eq!(
            parse(["maa", "roguelike", "Mizuki", "--refresh-trader-with-dice"]).unwrap(),
            default_params.join(object!(
                "theme" => "Mizuki",
                "refresh_trader_with_dice" => true,
            )),
        );

        assert_eq!(
            parse([
                "maa",
                "roguelike",
                "Sami",
                "--use-foldartal",
                "-F英雄",
                "-F大地"
            ])
            .unwrap(),
            default_params.join(object!(
                "theme" => "Sami",
                "use_foldartal" => true,
                "start_foldartal_list" => MAAValue::Array(vec![
                    MAAValue::from("英雄"),
                    MAAValue::from("大地"),
                ]),
            )),
        );
        assert!(parse(["maa", "roguelike", "Sami", "--mode", "5"]).is_err());
        assert_eq!(
            parse([
                "maa",
                "roguelike",
                "Sami",
                "--mode=5",
                "-P目空一些",
                "-P图像损坏",
            ])
            .unwrap(),
            default_params.join(object!(
                "theme" => "Sami",
                "mode" => 5,
                "use_foldartal" => false,
                "check_collapsal_paradigms" => true,
                "double_check_collapsal_paradigms" => true,
                "expected_collapsal_paradigms" => MAAValue::Array(vec![
                    MAAValue::from("目空一些"),
                    MAAValue::from("图像损坏"),
                ]),
            )),
        );

        assert_eq!(
            parse([
                "maa",
                "roguelike",
                "Sarkaz",
                "--mode=1",
                "--start-with-seed",
                "--seed=123456",
            ])
            .unwrap(),
            default_params.join(object!(
                "theme" => "Sarkaz",
                "mode" => 1,
                "start_with_seed" => "123456",
            )),
        );

        // Test that seed without start_with_seed flag is an error
        assert!(parse([
            "maa",
            "roguelike",
            "Sarkaz",
            "--mode=1",
            "--start-with-seed",
        ])
        .is_err());
    }

    #[test]
    fn test_collectible_mode() {
        use super::super::IntoParameters;
        
        fn parse<I, T>(args: I) -> Result<MAAValue, anyhow::Error>
        where
            I: IntoIterator<Item = T>,
            T: Into<std::ffi::OsString> + Clone,
        {
            let command = parse_from(args).command;
            match command {
                Command::Roguelike { params, .. } => params.into_parameters_no_context(),
                _ => panic!("Not a Roguelike command"),
            }
        }

        // Test collectible mode with all options
        let result = parse([
            "maa",
            "roguelike",
            "Mizuki",
            "--mode=4",
            "--collectible-squad=突击战术分队",
            "--collectible-shopping",
            "--collectible-start-awards=hot_water,hope,idea",
            "--start-with-elite-two",
        ])
        .unwrap();

        assert_eq!(result.get("mode").unwrap(), &MAAValue::from(4));
        assert_eq!(
            result.get("collectible_mode_squad").unwrap(),
            &MAAValue::from("突击战术分队")
        );
        assert_eq!(
            result.get("collectible_mode_shopping").unwrap(),
            &MAAValue::from(true)
        );
        assert_eq!(
            result.get("start_with_elite_two").unwrap(),
            &MAAValue::from(true)
        );

        // Verify collectible_mode_start_list
        let start_list = result.get("collectible_mode_start_list").unwrap();
        if let MAAValue::Object(obj) = start_list {
            assert_eq!(obj.get("hot_water").unwrap(), &MAAValue::from(true));
            assert_eq!(obj.get("hope").unwrap(), &MAAValue::from(true));
            assert_eq!(obj.get("ideas").unwrap(), &MAAValue::from(true)); // Note: idea -> ideas
        } else {
            panic!("Expected Object for collectible_mode_start_list");
        }

        // Test with invalid award name (should be ignored with warning)
        let result = parse([
            "maa",
            "roguelike",
            "Sami",
            "--mode=4",
            "--collectible-start-awards=hot_water,invalid,shield",
        ])
        .unwrap();

        let start_list = result.get("collectible_mode_start_list").unwrap();
        if let MAAValue::Object(obj) = start_list {
            assert_eq!(obj.get("hot_water").unwrap(), &MAAValue::from(true));
            assert_eq!(obj.get("shield").unwrap(), &MAAValue::from(true));
            assert!(obj.get("invalid").is_none());
        } else {
            panic!("Expected Object");
        }
    }

    #[test]
    fn test_monthly_squad_mode() {
        use super::super::IntoParameters;
        
        fn parse<I, T>(args: I) -> Result<MAAValue, anyhow::Error>
        where
            I: IntoIterator<Item = T>,
            T: Into<std::ffi::OsString> + Clone,
        {
            let command = parse_from(args).command;
            match command {
                Command::Roguelike { params, .. } => params.into_parameters_no_context(),
                _ => panic!("Not a Roguelike command"),
            }
        }

        let result = parse([
            "maa",
            "roguelike",
            "JieGarden",
            "--mode=6",
            "--monthly-squad-auto-iterate",
            "--monthly-squad-check-comms",
        ])
        .unwrap();

        assert_eq!(result.get("mode").unwrap(), &MAAValue::from(6));
        assert_eq!(
            result.get("monthly_squad_auto_iterate").unwrap(),
            &MAAValue::from(true)
        );
        assert_eq!(
            result.get("monthly_squad_check_comms").unwrap(),
            &MAAValue::from(true)
        );
    }

    #[test]
    fn test_deep_exploration_mode() {
        use super::super::IntoParameters;
        
        fn parse<I, T>(args: I) -> Result<MAAValue, anyhow::Error>
        where
            I: IntoIterator<Item = T>,
            T: Into<std::ffi::OsString> + Clone,
        {
            let command = parse_from(args).command;
            match command {
                Command::Roguelike { params, .. } => params.into_parameters_no_context(),
                _ => panic!("Not a Roguelike command"),
            }
        }

        let result = parse([
            "maa",
            "roguelike",
            "JieGarden",
            "--mode=7",
            "--deep-exploration-auto-iterate",
        ])
        .unwrap();

        assert_eq!(result.get("mode").unwrap(), &MAAValue::from(7));
        assert_eq!(
            result.get("deep_exploration_auto_iterate").unwrap(),
            &MAAValue::from(true)
        );
    }

    #[test]
    fn test_jiegarden_find_playtime() {
        use super::super::IntoParameters;
        
        fn parse<I, T>(args: I) -> Result<MAAValue, anyhow::Error>
        where
            I: IntoIterator<Item = T>,
            T: Into<std::ffi::OsString> + Clone,
        {
            let command = parse_from(args).command;
            match command {
                Command::Roguelike { params, .. } => params.into_parameters_no_context(),
                _ => panic!("Not a Roguelike command"),
            }
        }

        // Valid targets: 1, 2, 3
        for target in 1..=3 {
            let result = parse([
                "maa",
                "roguelike",
                "JieGarden",
                "--mode=20001",
                &format!("--find-playtime-target={}", target),
            ])
            .unwrap();

            assert_eq!(result.get("mode").unwrap(), &MAAValue::from(20001));
            assert_eq!(
                result.get("find_playTime_target").unwrap(),
                &MAAValue::from(target)
            );
        }

        // Invalid target: 0
        assert!(parse([
            "maa",
            "roguelike",
            "JieGarden",
            "--mode=20001",
            "--find-playtime-target=0",
        ])
        .is_err());

        // Invalid target: 4
        assert!(parse([
            "maa",
            "roguelike",
            "JieGarden",
            "--mode=20001",
            "--find-playtime-target=4",
        ])
        .is_err());

        // Missing target
        assert!(parse(["maa", "roguelike", "JieGarden", "--mode=20001",]).is_err());

        // Mode 20001 only for JieGarden
        assert!(parse([
            "maa",
            "roguelike",
            "Sarkaz",
            "--mode=20001",
            "--find-playtime-target=1",
        ])
        .is_err());
    }

    #[test]
    fn test_sami_advanced_params() {
        use super::super::IntoParameters;
        
        fn parse<I, T>(args: I) -> Result<MAAValue, anyhow::Error>
        where
            I: IntoIterator<Item = T>,
            T: Into<std::ffi::OsString> + Clone,
        {
            let command = parse_from(args).command;
            match command {
                Command::Roguelike { params, .. } => params.into_parameters_no_context(),
                _ => panic!("Not a Roguelike command"),
            }
        }

        // Test first floor foldartal
        let result = parse([
            "maa",
            "roguelike",
            "Sami",
            "--mode=4",
            "--sami-first-floor-foldartal",
            "--sami-first-floor-foldartals=板子1",
            "--sami-first-floor-foldartals=板子2",
        ])
        .unwrap();

        let foldartal_list = result.get("first_floor_foldartal").unwrap();
        if let MAAValue::Array(arr) = foldartal_list {
            assert_eq!(arr.len(), 2);
            assert_eq!(arr[0], MAAValue::from("板子1"));
            assert_eq!(arr[1], MAAValue::from("板子2"));
        } else {
            panic!("Expected Array");
        }

        // Test new squad2 starting foldartal
        let result = parse([
            "maa",
            "roguelike",
            "Sami",
            "--sami-new-squad2-starting-foldartal",
            "--sami-new-squad2-starting-foldartals=远见A",
            "--sami-new-squad2-starting-foldartals=远见B",
        ])
        .unwrap();

        let foldartal_list = result.get("start_foldartal_list").unwrap();
        if let MAAValue::Array(arr) = foldartal_list {
            assert_eq!(arr.len(), 2);
            assert_eq!(arr[0], MAAValue::from("远见A"));
            assert_eq!(arr[1], MAAValue::from("远见B"));
        } else {
            panic!("Expected Array");
        }

        // Test that new squad2 takes precedence over general start_foldartals
        let result = parse([
            "maa",
            "roguelike",
            "Sami",
            "--sami-new-squad2-starting-foldartal",
            "--sami-new-squad2-starting-foldartals=优先",
            "-F不应该出现",
        ])
        .unwrap();

        let foldartal_list = result.get("start_foldartal_list").unwrap();
        if let MAAValue::Array(arr) = foldartal_list {
            assert_eq!(arr.len(), 1);
            assert_eq!(arr[0], MAAValue::from("优先"));
        } else {
            panic!("Expected Array");
        }
    }

    #[test]
    fn test_stop_conditions() {
        use super::super::IntoParameters;
        
        fn parse<I, T>(args: I) -> Result<MAAValue, anyhow::Error>
        where
            I: IntoIterator<Item = T>,
            T: Into<std::ffi::OsString> + Clone,
        {
            let command = parse_from(args).command;
            match command {
                Command::Roguelike { params, .. } => params.into_parameters_no_context(),
                _ => panic!("Not a Roguelike command"),
            }
        }

        let result = parse([
            "maa",
            "roguelike",
            "Phantom",
            "--stop-when-deposit-full",
            "--stop-when-level-max",
        ])
        .unwrap();

        assert_eq!(
            result.get("stop_when_deposit_full").unwrap(),
            &MAAValue::from(true)
        );
        assert_eq!(
            result.get("stop_at_max_level").unwrap(),
            &MAAValue::from(true)
        );
    }
}
