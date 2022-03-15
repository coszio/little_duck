
pub enum Programa {
  WithVars(Id, Vec<Var>, Bloque),
  WithoutVars(Id, Bloque),
}

#[derive(Clone)]
pub enum Id {
  Id(String),
  Error,
}

pub enum Var {
  Var(Id, Tipo),
  Error,
}

#[derive(Clone, Copy)]
pub enum Tipo {
  Int,
  Float,
}

pub enum Bloque {
  Bloque(Vec<Estatuto>),
  Error
}

pub enum Estatuto {
  Asignacion(Id, Box<Expresion>),
  Condicion(Box<Expresion>, Vec<Bloque>),
  Escritura(Vec<PrintArg>),
}

pub enum PrintArg {
  Expresion(Box<Expresion>),
  CteString(String),
}

pub enum Expresion {
  Simple(Box<Exp>),
  Compuesta(Box<Exp>, CmpOp, Box<Exp>),
}

pub enum Exp {
  Simple(Box<Termino>),
  Compuesta(Box<Termino>, Sign, Box<Exp>)
}

pub enum CmpOp {
  GT,
  LT,
  NE,
}

pub enum Sign {
  PLUS,
  MINUS,
}

pub enum Termino {
  Simple(Factor),
  Compuesto(Factor, FacOp, Box<Termino>),
}

pub enum FacOp {
  MUL,
  DIV,
}

pub enum Factor {
  Expresion(Box<Expresion>),
  SignedVar(Sign, VarCte),
  UnsignedVar(VarCte),
}

pub enum VarCte {
  Id(Id),
  Cte(String),
}
