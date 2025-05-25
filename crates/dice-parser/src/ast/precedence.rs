use super::Expr;

type Prec = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Empty,
    Not,
    Label,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl TryFrom<Expr<'_>> for Op {
    type Error = ();

    fn try_from(value: Expr<'_>) -> Result<Self, Self::Error> {
        match value {
            Expr::Int(_) => Err(()),
            Expr::Dice(_, _) => Err(()),
            Expr::Not(..) => Ok(Op::Not),
            Expr::Label(..) => Ok(Op::Label),
            Expr::Add(..) => Ok(Op::Add),
            Expr::Sub(..) => Ok(Op::Sub),
            Expr::Mul(..) => Ok(Op::Mul),
            Expr::Div(..) => Ok(Op::Div),
        }
    }
}

impl Op {
    pub const fn from_expr(expr: &Expr<'_>) -> Self {
        match expr {
            Expr::Int(_) | Expr::Dice(_, _) => Self::Empty,
            Expr::Not(..) => Self::Not,
            Expr::Label(..) => Self::Label,
            Expr::Add(..) => Self::Add,
            Expr::Sub(..) => Self::Sub,
            Expr::Mul(..) => Self::Mul,
            Expr::Div(..) => Self::Div,
        }
    }

    pub const fn prec(&self) -> Prec {
        match self {
            Op::Not => 20,
            Op::Label => 10,
            Op::Add => 40,
            Op::Sub => 40,
            Op::Mul => 30,
            Op::Div => 30,
            Op::Empty => 0,
        }
    }

    pub const fn as_binop(&self) -> Option<BinOp> {
        match self {
            Op::Empty | Op::Not | Op::Label => None,
            Op::Add => Some(BinOp::Add),
            Op::Sub => Some(BinOp::Sub),
            Op::Mul => Some(BinOp::Mul),
            Op::Div => Some(BinOp::Div),
        }
    }
}

impl BinOp {
    pub const fn as_str(&self) -> &'static str {
        match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
        }
    }

    pub const fn as_op(&self) -> Op {
        match self {
            BinOp::Add => Op::Add,
            BinOp::Sub => Op::Sub,
            BinOp::Mul => Op::Mul,
            BinOp::Div => Op::Div,
        }
    }

    pub const fn prec(&self) -> Prec {
        self.as_op().prec()
    }

    pub const fn needs_parenthesis(me: BinOp, lop: Op, rop: Op) -> (bool, bool) {
        let lop = lop.prec();
        let rop = rop.prec()
            + match me {
                BinOp::Add => 0,
                BinOp::Sub => 1,
                BinOp::Mul => 0,
                BinOp::Div => 1,
            };
        let me = me.prec();

        (me < lop, me < rop)
    }
}
