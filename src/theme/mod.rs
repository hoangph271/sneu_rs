
#[derive(PartialEq, Eq, Default, Clone)]
pub enum ColorVariant {
    #[default]
    Primary,
    Warning,
    Danger,
}

impl ColorVariant {
    pub fn bg(&self) -> String {
        match self {
            ColorVariant::Primary => "bg-sky-500 hover:bg-sky-700",
            ColorVariant::Warning => "bg-amber-500 hover:bg-amber-700",
            ColorVariant::Danger => "bg-rose-500 hover:bg-rose-700",
        }
        .to_owned()
    }
}

pub trait Themable {
    fn sub_theme(&self) -> Option<Box<dyn Themable>>;

    fn color_variant(&self) -> ColorVariant {
        ColorVariant::default()
    }

    fn classnames(&self) -> String {
        let sub_classnames = self.sub_theme().map(|themable| themable.classnames());

        [sub_classnames]
            .into_iter()
            .flatten()
            .collect::<Vec<String>>()
            .join(" ")
    }
}
