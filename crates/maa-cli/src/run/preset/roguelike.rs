use anyhow::bail;
use clap::ValueEnum;
use maa_value::prelude::*;

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
    /// 0: mode for score;
    /// 1: mode for ingots;
    /// 2: combination of 0 and 1, deprecated;
    /// 3: mode for pass, not implemented yet;
    /// 4: mode that exist after 3rd floor;
    /// 5: mode for collapsal paradigms, only for Sami, use with `expected_collapsal_paradigms`
    /// 6: monthly squad mode;
    /// 7: exploration mode (Exploration);
    /// 10001: Sarkaz fast pass mode (FastPass, Sarkaz only);
    /// 20001: JieGarden find playtime mode (FindPlaytime, JieGarden only);
    #[arg(long, default_value = "0")]
    mode: i32,

    // TODO: input localized names, maybe during initialization of tasks

    // Start related parameters
    /// Squad to start with in Chinese, e.g. "指挥分队" (default), "后勤分队"
    #[arg(long)]
    squad: Option<String>,
    /// Starting core operator in Chinese, e.g. "维什戴尔"
    #[arg(long)]
    core_char: Option<String>,
    /// Starting operators recruitment combination in Chinese, e.g. "取长补短", "先手必胜"
    /// (default)
    #[arg(long)]
    roles: Option<String>,

    /// Stop after given count, if not given, never stop
    #[arg(long, alias = "start-count")]
    starts_count: Option<i32>,

    /// Whether to purchase in collectible (mode 4)
    #[arg(long)]
    collectible_mode_shopping: bool,

    /// First floor foldartal target for Sami collectible mode
    #[arg(long)]
    first_floor_foldartal: Option<String>,

    /// Target playtime type for JieGarden FindPlaytime mode
    ///
    /// 1: 令(Ling)
    /// 2: 黍(Shu)
    /// 3: 年(Nian)
    #[arg(long = "find-playTime-target")]
    find_play_time_target: Option<i32>,

    /// Difficulty, not valid for Phantom theme (no numerical difficulty)
    ///
    /// If the given difficulty is larger than the maximum difficulty of the theme, it will be
    /// capped to the maximum difficulty. If not given, 0 will be used.
    #[arg(long)]
    difficulty: Option<i32>,

    // Investment related parameters
    /// Disable investment
    #[arg(long)]
    disable_investment: bool,
    /// Try to gain more score in investment mode
    ///
    /// By default, some actions will be skipped in investment mode to save time.
    /// If this option is enabled, try to gain exp score in investment mode.
    #[arg(long)]
    investment_with_more_score: bool,
    /// Stop exploration investment reaches given count
    #[arg(long)]
    investments_count: Option<i32>,
    /// Do not stop exploration when investment is full
    #[arg(long)]
    no_stop_when_investment_full: bool,

    // Support related parameters
    /// Use support operator
    #[arg(long)]
    use_support: bool,
    /// Use non-friend support operator
    #[arg(long)]
    use_nonfriend_support: bool,

    // Elite related parameters
    /// Start with elite two
    #[arg(long)]
    start_with_elite_two: bool,
    /// Only start with elite two
    #[arg(long)]
    only_start_with_elite_two: bool,

    /// Stop exploration before final boss
    #[arg(long)]
    stop_at_final_boss: bool,

    /// Stop when roguelike level is maxed
    #[arg(long)]
    stop_at_max_level: bool,

    // Mizuki specific parameters
    /// Whether to refresh trader with dice (only available in Mizuki theme)
    #[arg(long)]
    refresh_trader_with_dice: bool,

    // Sami specific parameters
    // Foldartal related parameters
    /// Whether to use Foldartal in Sami theme
    #[arg(long)]
    use_foldartal: bool,
    /// A list of expected Foldartal to be started with
    #[arg(short = 'F', long)]
    start_foldartals: Vec<String>,
    /// A list of expected collapsal paradigms
    #[arg(short = 'P', long)]
    expected_collapsal_paradigms: Vec<String>,

    /// Whether to check collapsal paradigms (Sami only)
    #[arg(long)]
    check_collapsal_paradigms: Option<bool>,

    /// Whether to enable additional anti-miss checks for collapsal paradigms
    #[arg(long)]
    double_check_collapsal_paradigms: Option<bool>,

    /// Automatically iterate monthly squad rewards (mode 6)
    #[arg(long)]
    monthly_squad_auto_iterate: bool,

    /// Include monthly squad communications when auto iterating (mode 6)
    #[arg(long)]
    monthly_squad_check_comms: bool,

    /// Automatically iterate deep exploration rewards (mode 7)
    #[arg(long)]
    deep_exploration_auto_iterate: bool,

    /// Squad used in collectible mode (mode 4)
    #[arg(long)]
    collectible_mode_squad: Option<String>,

    /// Collectible mode expected start reward: hot water
    #[arg(long)]
    collectible_start_hot_water: bool,

    /// Collectible mode expected start reward: shield
    #[arg(long)]
    collectible_start_shield: bool,

    /// Collectible mode expected start reward: ingot
    #[arg(long)]
    collectible_start_ingot: bool,

    /// Collectible mode expected start reward: hope
    #[arg(long)]
    collectible_start_hope: bool,

    /// Collectible mode expected start reward: random collectible
    #[arg(long)]
    collectible_start_random: bool,

    /// Collectible mode expected start reward: key (Mizuki)
    #[arg(long)]
    collectible_start_key: bool,

    /// Collectible mode expected start reward: dice (Mizuki)
    #[arg(long)]
    collectible_start_dice: bool,

    /// Collectible mode expected start reward: ideas (Sarkaz)
    #[arg(long)]
    collectible_start_ideas: bool,

    /// Collectible mode expected start reward: ticket (JieGarden)
    #[arg(long)]
    collectible_start_ticket: bool,

    // Sarkaz specific parameters
    /// Whether to start with seed, only available in Sarkaz theme and mode 1
    #[arg(long)]
    start_with_seed: bool,
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
            10001 if !matches!(theme, Theme::Sarkaz) => {
                bail!("Mode 10001 is only available in Sarkaz theme");
            }
            20001 if !matches!(theme, Theme::JieGarden) => {
                bail!("Mode 20001 is only available in JieGarden theme");
            }
            0..=7 | 10001 | 20001 => {}
            _ => bail!("Mode must be in supported values (0-7, 10001, 20001)"),
        }

        let mut value = object!(
            "theme" => self.theme.to_str(),
            "mode" => self.mode,
            "squad" =>? self.squad,
            "roles" =>? self.roles,
            "core_char" =>? self.core_char,
            "starts_count" =>? self.starts_count,
            "stop_at_final_boss" => self.stop_at_final_boss,
            "stop_at_max_level" => self.stop_at_max_level,
            "monthly_squad_auto_iterate" => self.monthly_squad_auto_iterate,
            "monthly_squad_check_comms" => self.monthly_squad_check_comms,
            "deep_exploration_auto_iterate" => self.deep_exploration_auto_iterate,
            "collectible_mode_squad" =>? self.collectible_mode_squad,
        );

        if self.collectible_start_hot_water
            || self.collectible_start_shield
            || self.collectible_start_ingot
            || self.collectible_start_hope
            || self.collectible_start_random
            || self.collectible_start_key
            || self.collectible_start_dice
            || self.collectible_start_ideas
            || self.collectible_start_ticket
        {
            value.insert(
                "collectible_mode_start_list",
                object!(
                    "hot_water" => self.collectible_start_hot_water,
                    "shield" => self.collectible_start_shield,
                    "ingot" => self.collectible_start_ingot,
                    "hope" => self.collectible_start_hope,
                    "random" => self.collectible_start_random,
                    "key" => self.collectible_start_key,
                    "dice" => self.collectible_start_dice,
                    "ideas" => self.collectible_start_ideas,
                    "ticket" => self.collectible_start_ticket,
                )
                .into(),
            );
        }

        if self.collectible_mode_shopping {
            value.insert("collectible_mode_shopping", true.into());
        }
        if let Some(first_floor_foldartal) = self.first_floor_foldartal
            && !first_floor_foldartal.is_empty()
        {
            value.insert("first_floor_foldartal", first_floor_foldartal.into());
        }
        if let Some(find_play_time_target) = self.find_play_time_target {
            value.insert("find_playTime_target", find_play_time_target.into());
        }

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
        if self.use_support {
            insert!(value,
                "use_support" => true,
                "use_nonfriend_support" => self.use_nonfriend_support
            );
        }

        // Elite settings
        if self.start_with_elite_two {
            insert!(value,
                "start_with_elite_two" => true,
                "only_start_with_elite_two" => self.only_start_with_elite_two
            );
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
                if !self.start_foldartals.is_empty() {
                    insert!(value, "start_foldartal_list" => self.start_foldartals?);
                }

                if let Some(check) = self.check_collapsal_paradigms {
                    value.insert("check_collapsal_paradigms", check.into());
                } else if mode == 5 {
                    value.insert("check_collapsal_paradigms", true.into());
                }

                if let Some(double_check) = self.double_check_collapsal_paradigms {
                    value.insert("double_check_collapsal_paradigms", double_check.into());
                } else if mode == 5 {
                    value.insert("double_check_collapsal_paradigms", true.into());
                }

                if !self.expected_collapsal_paradigms.is_empty() {
                    insert!(
                        value,
                        "expected_collapsal_paradigms" => self.expected_collapsal_paradigms?,
                    );
                }
            }
            Theme::Sarkaz if mode == 1 => {
                insert!(value, "start_with_seed" => self.start_with_seed);
            }
            _ => {}
        }

        Ok(value)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {

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
            "stop_at_max_level" => false,
            "monthly_squad_auto_iterate" => false,
            "monthly_squad_check_comms" => false,
            "deep_exploration_auto_iterate" => false,
        );

        assert_eq!(
            parse(["maa", "roguelike", "Phantom"]).unwrap(),
            default_params.join(object!("theme" => "Phantom")),
        );
        assert!(parse(["maa", "roguelike", "Phantom", "--mode", "5"]).is_err());
        assert!(parse(["maa", "roguelike", "Phantom", "--mode", "7"]).is_ok());

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
                "starts_count" => 100,
                "difficulty" => 15,
            )),
        );

        assert_eq!(
            parse([
                "maa",
                "roguelike",
                "Sarkaz",
                "--starts-count=150",
                "--difficulty=7",
            ])
            .unwrap(),
            default_params.join(object!(
                "theme" => "Sarkaz",
                "starts_count" => 150,
                "difficulty" => 7,
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
                "stop_at_max_level" => false,
                "monthly_squad_auto_iterate" => false,
                "monthly_squad_check_comms" => false,
                "deep_exploration_auto_iterate" => false,
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
        assert_eq!(
            parse(["maa", "roguelike", "Sami", "--mode", "5"]).unwrap(),
            default_params.join(object!(
                "theme" => "Sami",
                "mode" => 5,
                "use_foldartal" => false,
                "check_collapsal_paradigms" => true,
                "double_check_collapsal_paradigms" => true,
            )),
        );
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
            ])
            .unwrap(),
            default_params.join(object!(
                "theme" => "Sarkaz",
                "mode" => 1,
                "start_with_seed" => true,
            )),
        );

        assert_eq!(
            parse([
                "maa",
                "roguelike",
                "Sami",
                "--mode=4",
                "--collectible-mode-shopping",
                "--first-floor-foldartal",
                "英雄",
                "--use-foldartal",
                "--collectible-mode-squad",
                "生活至上分队",
                "--collectible-start-hot-water",
                "--collectible-start-random",
                "-F英雄",
            ])
            .unwrap(),
            default_params.join(object!(
                "theme" => "Sami",
                "mode" => 4,
                "collectible_mode_shopping" => true,
                "collectible_mode_squad" => "生活至上分队",
                "collectible_mode_start_list" => object!(
                    "hot_water" => true,
                    "shield" => false,
                    "ingot" => false,
                    "hope" => false,
                    "random" => true,
                    "key" => false,
                    "dice" => false,
                    "ideas" => false,
                    "ticket" => false,
                ),
                "first_floor_foldartal" => "英雄",
                "use_foldartal" => true,
                "start_foldartal_list" => MAAValue::Array(vec![MAAValue::from("英雄")]),
            )),
        );

        assert_eq!(
            parse([
                "maa",
                "roguelike",
                "Sami",
                "--check-collapsal-paradigms",
                "true",
                "--double-check-collapsal-paradigms",
                "false",
                "-P目空一些",
            ])
            .unwrap(),
            default_params.join(object!(
                "theme" => "Sami",
                "use_foldartal" => false,
                "check_collapsal_paradigms" => true,
                "double_check_collapsal_paradigms" => false,
                "expected_collapsal_paradigms" => MAAValue::Array(vec![MAAValue::from("目空一些")]),
            )),
        );

        assert_eq!(
            parse([
                "maa",
                "roguelike",
                "JieGarden",
                "--mode",
                "20001",
                "--find-playTime-target",
                "2",
            ])
            .unwrap(),
            default_params.join(object!(
                "theme" => "JieGarden",
                "mode" => 20001,
                "find_playTime_target" => 2,
            )),
        );

        assert_eq!(
            parse(["maa", "roguelike", "Sarkaz", "--mode", "10001",]).unwrap(),
            default_params.join(object!(
                "theme" => "Sarkaz",
                "mode" => 10001,
            )),
        );
    }
}
