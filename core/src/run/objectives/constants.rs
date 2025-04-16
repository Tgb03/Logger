pub const MAIN_ONLY: (bool, bool) = (false, false);
pub const SECD_ONLY: (bool, bool) = (true, false);
pub const OVRL_ONLY: (bool, bool) = (false, true);
pub const PE_OBJECT: (bool, bool) = (true, true);

pub const OPTIONALS_ALL: [(bool, bool); 83] = [
    MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY,
    MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY,
    MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, SECD_ONLY,
    PE_OBJECT, PE_OBJECT, SECD_ONLY, PE_OBJECT, PE_OBJECT, SECD_ONLY, PE_OBJECT, PE_OBJECT,
    SECD_ONLY, PE_OBJECT, SECD_ONLY, SECD_ONLY, PE_OBJECT, PE_OBJECT, PE_OBJECT, PE_OBJECT,
    SECD_ONLY, MAIN_ONLY, SECD_ONLY, PE_OBJECT, SECD_ONLY, SECD_ONLY, MAIN_ONLY, MAIN_ONLY,
    MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, SECD_ONLY, SECD_ONLY, MAIN_ONLY, SECD_ONLY, PE_OBJECT,
    MAIN_ONLY, MAIN_ONLY, SECD_ONLY, PE_OBJECT, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, SECD_ONLY,
    PE_OBJECT, MAIN_ONLY, PE_OBJECT, PE_OBJECT, OVRL_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY,
    SECD_ONLY, SECD_ONLY, SECD_ONLY, SECD_ONLY, SECD_ONLY, PE_OBJECT, MAIN_ONLY, MAIN_ONLY,
    SECD_ONLY, SECD_ONLY, SECD_ONLY,
];

pub const LEVELS_ALL: [&str; 83] = [
    "R1A1", "R1B1", "R1B2", "R1C1", "R1C2", "R1D1", "R2A1", "R2B1", "R2B2", "R2B3", "R2B4", "R2C1",
    "R2C2", "R2D1", "R2D2", "R2E1", "R3A1", "R3A2", "R3A3", "R3B1", "R3B2", "R3C1", "R3D1", "R4A1",
    "R4A2", "R4A3", "R4B1", "R4B2", "R4B3", "R4C1", "R4C2", "R4C3", "R4D1", "R4D2", "R4E1", "R5A1",
    "R5A2", "R5A3", "R5B1", "R5B2", "R5B3", "R5B4", "R5C1", "R5C2", "R5C3", "R5D1", "R5D2", "R5E1",
    "R6A1", "R6A2", "R6B1", "R6B2", "R6B4", "R6C1", "R6C2", "R6C3", "R6C4", "R6D1", "R6D2", "R6D3",
    "R6D4", "R7A1", "R7B1", "R7B2", "R7B3", "R7C1", "R7C2", "R7C3", "R7D1", "R7D2", "R7E1", "R8A1",
    "R8A3", "R8B1", "R8B2", "R8B3", "R8B4", "R8C1", "R8C3", "R8D1", "R8D3", "R8E1", "R8E3",
];
