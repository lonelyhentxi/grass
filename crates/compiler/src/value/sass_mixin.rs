use std::fmt;
use std::sync::Arc;
use crate::ast::AstMixin;
use crate::evaluate::Environment;
use crate::builtin::BuiltinMixin;
use crate::common::Identifier;

#[derive(Clone, Eq, PartialEq)]
pub enum SassMixin {
    UserDefined(UserDefinedMixin),
    Builtin(BuiltinMixin, Identifier),
}

impl SassMixin {
    /// Get the name of the mixin referenced
    ///
    /// Used mainly in debugging and `inspect()`
    pub fn name(&self) -> Identifier {
        match self {
            Self::Builtin(_, name)
            | Self::UserDefined(UserDefinedMixin { name, .. }, ..) => *name
        }
    }

    /// Whether the function is builtin or user-defined
    ///
    /// Used only in `std::fmt::Debug` for `SassFunction`
    fn kind(&self) -> &'static str {
        match &self {
            Self::Builtin(..) => "Builtin",
            Self::UserDefined { .. } => "UserDefined",
        }
    }
}

impl fmt::Debug for SassMixin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserDefined(u, ..) => f
                .debug_struct("UserDefinedMixin")
                .field("name", &self.name())
                .field("kind", &self.kind())
                .field("args", &u.mixin.args)
                .field("body", &u.mixin.body)
                .field("has_content", &u.mixin.has_content)
                .finish(),
            Self::Builtin(..) => f
                .debug_struct("BuiltinMixin")
                .field("name", &self.name())
                .field("kind", &self.kind())
                .finish(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserDefinedMixin {
    pub(crate) mixin: Arc<AstMixin>,
    pub name: Identifier,
    pub(crate) env: Environment,
}

impl PartialEq for UserDefinedMixin {
    fn eq(&self, other: &Self) -> bool {
        self.mixin == other.mixin && self.name == other.name
    }
}

impl Eq for UserDefinedMixin {}
