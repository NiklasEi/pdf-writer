use super::*;

/// Writer for a _transition dictionary_.
///
/// This struct is created by [`Page::trans`].
pub struct Transition<'a> {
    dict: Dict<'a>,
}

impl<'a> Transition<'a> {
    pub(crate) fn new(obj: Obj<'a>) -> Self {
        let mut dict = obj.dict();
        dict.pair(Name(b"Type"), Name(b"Trans"));
        Self { dict }
    }

    /// Write the `/S` attribute to set the transition style.
    pub fn style(&mut self, kind: TransitionStyle) -> &mut Self {
        self.pair(Name(b"S"), kind.to_name());
        self
    }

    /// Write the `/D` attribute to set the transition duration.
    pub fn duration(&mut self, seconds: f32) -> &mut Self {
        self.pair(Name(b"D"), seconds);
        self
    }

    /// Write the `/Dm` attribute to set the transition direction. Will be
    /// horizontal if the argument is `false`.
    pub fn dimension(&mut self, vertical: bool) -> &mut Self {
        let name = if vertical { Name(b"V") } else { Name(b"H") };

        self.pair(Name(b"Dm"), name);
        self
    }

    /// Write the `/M` attribute to set the transition direction. Will be
    /// inwards if the argument is `false`.
    pub fn direction(&mut self, outward: bool) -> &mut Self {
        let name = if outward { Name(b"O") } else { Name(b"I") };

        self.pair(Name(b"M"), name);
        self
    }

    /// Write the `/Di` attribute to set the transition angle.
    pub fn angle(&mut self, angle: TransitionAngle) -> &mut Self {
        angle.write_to_obj(self.key(Name(b"Di")));
        self
    }

    /// Write the `/SS` attribute to set the scale for the `Fly` transition.
    /// (1.5+)
    pub fn scale(&mut self, scale: f32) -> &mut Self {
        self.pair(Name(b"SS"), scale);
        self
    }

    /// Write the `/B` attribute for the `Fly` transition. (1.5+)
    pub fn opaque(&mut self, opaque: f32) -> &mut Self {
        self.pair(Name(b"F"), opaque);
        self
    }
}

deref!('a, Transition<'a> => Dict<'a>, dict);

/// The kind of transition.
pub enum TransitionStyle {
    /// Split the slide down the middle.
    Split,
    /// Multiple lines roll up the slide.
    Blinds,
    /// The new slide is revealed in a growing box.
    Box,
    /// Single line that sweeps across the slide.
    Wipe,
    /// Slide dissolves gradually.
    Dissolve,
    /// Like dissolve, but starts on one side.
    Glitter,
    /// No effect.
    R,
    /// Changes are flown in. (1.5+)
    Fly,
    /// Old page slides out, new page slides in. (1.5+)
    Push,
    /// New page slides in to cover the old one. (1.5+)
    Cover,
    /// Old page slides out to uncover the new one. (1.5+)
    Uncover,
    /// A cross-fade. (1.5+)
    Fade,
}

impl TransitionStyle {
    fn to_name(self) -> Name<'static> {
        match self {
            Self::Split => Name(b"Split"),
            Self::Blinds => Name(b"Blinds"),
            Self::Box => Name(b"Box"),
            Self::Wipe => Name(b"Wipe"),
            Self::Dissolve => Name(b"Dissolve"),
            Self::Glitter => Name(b"Glitter"),
            Self::R => Name(b"R"),
            Self::Fly => Name(b"Fly"),
            Self::Push => Name(b"Push"),
            Self::Cover => Name(b"Cover"),
            Self::Uncover => Name(b"Uncover"),
            Self::Fade => Name(b"Fade"),
        }
    }
}

/// The angle at which the transition plays.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[allow(missing_docs)]
pub enum TransitionAngle {
    LeftToRight,
    BottomToTop,
    RightToLeft,
    TopToBottom,
    TopLeftToBottomRight,
    /// No direction in the `Fly` style.
    None,
}

impl TransitionAngle {
    fn write_to_obj(&self, obj: Obj<'_>) {
        match self {
            Self::LeftToRight => obj.primitive(0),
            Self::BottomToTop => obj.primitive(90),
            Self::RightToLeft => obj.primitive(180),
            Self::TopToBottom => obj.primitive(270),
            Self::TopLeftToBottomRight => obj.primitive(315),
            Self::None => obj.primitive(Name(b"None")),
        }
    }
}

/// Writer for an _action dictionary_.
///
/// This struct is created by [`Annotation::action`].
pub struct Action<'a> {
    dict: Dict<'a>,
}

impl<'a> Action<'a> {
    pub(crate) fn new(obj: Obj<'a>) -> Self {
        let mut dict = obj.dict();
        dict.pair(Name(b"Type"), Name(b"Action"));
        Self { dict }
    }

    /// Write the `/S` attribute to set the action type.
    pub fn action_type(&mut self, kind: ActionType) -> &mut Self {
        self.pair(Name(b"S"), kind.to_name());
        self
    }

    /// Start writing the `/D` attribute to set the destination of this
    /// GoTo-type action.
    pub fn dest_direct(&mut self, page: Ref) -> Destination<'_> {
        Destination::start(self.key(Name(b"D")), page)
    }

    /// Write the `/D` attribute to set the destination of this GoTo-type action
    /// to a named destination.
    pub fn dest_name(&mut self, name: Name) -> &mut Self {
        self.pair(Name(b"D"), name);
        self
    }

    /// Start writing the `/F` attribute, setting which file to go to or which
    /// application to launch.
    pub fn file(&mut self) -> FileSpec<'_> {
        FileSpec::new(self.key(Name(b"F")))
    }

    /// Write the `/NewWindow` attribute to set whether this remote GoTo action
    /// should open the referenced destination in another window.
    pub fn new_window(&mut self, new: bool) -> &mut Self {
        self.pair(Name(b"NewWindow"), new);
        self
    }

    /// Write the `/URI` attribute to set where this link action goes.
    pub fn uri(&mut self, uri: Str) -> &mut Self {
        self.pair(Name(b"URI"), uri);
        self
    }

    /// Write the `/IsMap` attribute to set if the click position of the user's
    /// cursor inside the link rectangle should be appended to the referenced
    /// URI as a query parameter.
    pub fn is_map(&mut self, map: bool) -> &mut Self {
        self.pair(Name(b"IsMap"), map);
        self
    }
}

deref!('a, Action<'a> => Dict<'a>, dict);

/// What kind of action to perform.
pub enum ActionType {
    /// Go to a destination in the document.
    GoTo,
    /// Go to a destination in another document.
    RemoteGoTo,
    /// Launch an application.
    Launch,
    /// Open a URI.
    Uri,
}

impl ActionType {
    fn to_name(self) -> Name<'static> {
        match self {
            Self::GoTo => Name(b"GoTo"),
            Self::RemoteGoTo => Name(b"GoToR"),
            Self::Launch => Name(b"Launch"),
            Self::Uri => Name(b"URI"),
        }
    }
}
