use std::os::raw::c_char;

// *** Types ***
// Header version number to test against
pub const SHARED_MEMORY_VERSION: usize = 13;
// Maximum allowed length of string
pub const STRING_LENGTH_MAX: usize = 64;
// Maximum number of general participant information allowed to be stored in memory-mapped file
pub const STORED_PARTICIPANTS_MAX: usize = 64;

pub const TYRE_COMPOUND_NAME_LENGTH_MAX: usize = 40;

// Tyres
#[repr(usize)]
pub enum TYRE_STATES {
    TYRE_FRONT_LEFT = 0,
    TYRE_FRONT_RIGHT,
    TYRE_REAR_LEFT,
    TYRE_REAR_RIGHT,
    //--------------
    TYRE_MAX,
}
// Vector
#[repr(usize)]
pub enum VECTOR_AXES {
    VEC_X = 0,
    VEC_Y,
    VEC_Z,
    //-------------
    VEC_MAX,
}

// (Type#1) GameState (to be used with 'mGameState')
pub enum GAME_STATES {
    GAME_EXITED = 0,
    GAME_FRONT_END,
    GAME_INGAME_PLAYING,
    GAME_INGAME_PAUSED,
    GAME_INGAME_INMENU_TIME_TICKING,
    GAME_INGAME_RESTARTING,
    GAME_INGAME_REPLAY,
    GAME_FRONT_END_REPLAY,
    //-------------
    GAME_MAX,
}

// (Type#2) Session state (to be used with 'mSessionState')
pub enum SESSION_STATES {
    SESSION_INVALID = 0,
    SESSION_PRACTICE,
    SESSION_TEST,
    SESSION_QUALIFY,
    SESSION_FORMATION_LAP,
    SESSION_RACE,
    SESSION_TIME_ATTACK,
    //-------------
    SESSION_MAX,
}

// (Type#3) RaceState (to be used with 'mRaceState' and 'mRaceStates')
pub enum RACE_STATES {
    RACESTATE_INVALID = 0,
    RACESTATE_NOT_STARTED,
    RACESTATE_RACING,
    RACESTATE_FINISHED,
    RACESTATE_DISQUALIFIED,
    RACESTATE_RETIRED,
    RACESTATE_DNF,
    //-------------
    RACESTATE_MAX,
}

// (Type#5) Flag Colours (to be used with 'mHighestFlagColour')
pub enum FLAG_COLORS {
    FLAG_COLOUR_NONE = 0, // Not used for actual flags, only for some query functions
    FLAG_COLOUR_GREEN,    // End of danger zone, or race started
    FLAG_COLOUR_BLUE,     // Faster car wants to overtake the participant
    FLAG_COLOUR_WHITE_SLOW_CAR, // Slow car in area
    FLAG_COLOUR_WHITE_FINAL_LAP, // Final Lap
    FLAG_COLOUR_RED, // Huge collisions where one or more cars become wrecked and block the track
    FLAG_COLOUR_YELLOW, // Danger on the racing surface itself
    FLAG_COLOUR_DOUBLE_YELLOW, // Danger that wholly or partly blocks the racing surface
    FLAG_COLOUR_BLACK_AND_WHITE, // Unsportsmanlike conduct
    FLAG_COLOUR_BLACK_ORANGE_CIRCLE, // Mechanical Failure
    FLAG_COLOUR_BLACK, // Participant disqualified
    FLAG_COLOUR_CHEQUERED, // Chequered flag
    //-------------
    FLAG_COLOUR_MAX,
}

// (Type#6) Flag Reason (to be used with 'mHighestFlagReason')
pub enum FLAG_REASONS {
    FLAG_REASON_NONE = 0,
    FLAG_REASON_SOLO_CRASH,
    FLAG_REASON_VEHICLE_CRASH,
    FLAG_REASON_VEHICLE_OBSTRUCTION,
    //-------------
    FLAG_REASON_MAX,
}

// (Type#7) Pit Mode (to be used with 'mPitMode')
#[repr(usize)]
pub enum PIT_MODES {
    PIT_MODE_NONE = 0,
    PIT_MODE_DRIVING_INTO_PITS,
    PIT_MODE_IN_PIT,
    PIT_MODE_DRIVING_OUT_OF_PITS,
    PIT_MODE_IN_GARAGE,
    PIT_MODE_DRIVING_OUT_OF_GARAGE,
    //-------------
    PIT_MODE_MAX,
}

// (Type#8) Pit Stop Schedule (to be used with 'mPitSchedule')
pub enum PIT_SCHEDULES {
    PIT_SCHEDULE_NONE = 0,           // Nothing scheduled
    PIT_SCHEDULE_PLAYER_REQUESTED,   // Used for standard pit sequence - requested by player
    PIT_SCHEDULE_ENGINEER_REQUESTED, // Used for standard pit sequence - requested by engineer
    PIT_SCHEDULE_DAMAGE_REQUESTED, // Used for standard pit sequence - requested by engineer for damage
    PIT_SCHEDULE_MANDATORY, // Used for standard pit sequence - requested by engineer from career enforced lap number
    PIT_SCHEDULE_DRIVE_THROUGH, // Used for drive-through penalty
    PIT_SCHEDULE_STOP_GO,   // Used for stop-go penalty
    PIT_SCHEDULE_PITSPOT_OCCUPIED, // Used for drive-through when pitspot is occupied
    //-------------
    PIT_SCHEDULE_MAX,
}

// (Type#9) Car Flags (to be used with 'mCarFlags')
pub enum CAR_FLAGS {
    CAR_HEADLIGHT = (1 << 0),
    CAR_ENGINE_ACTIVE = (1 << 1),
    CAR_ENGINE_WARNING = (1 << 2),
    CAR_SPEED_LIMITER = (1 << 3),
    CAR_ABS = (1 << 4),
    CAR_HANDBRAKE = (1 << 5),
    CAR_TCS = (1 << 6),
    CAR_SCS = (1 << 7),
}

pub enum TYRE_FLAGS {
    TYRE_ATTACHED = (1 << 0),
    TYRE_INFLATED = (1 << 1),
    TYRE_IS_ON_GROUND = (1 << 2),
}

// (Type#11) Terrain Materials (to be used with 'mTerrain')
pub enum TERRAIN_MATERIALS {
    TERRAIN_ROAD = 0,
    TERRAIN_LOW_GRIP_ROAD,
    TERRAIN_BUMPY_ROAD1,
    TERRAIN_BUMPY_ROAD2,
    TERRAIN_BUMPY_ROAD3,
    TERRAIN_MARBLES,
    TERRAIN_GRASSY_BERMS,
    TERRAIN_GRASS,
    TERRAIN_GRAVEL,
    TERRAIN_BUMPY_GRAVEL,
    TERRAIN_RUMBLE_STRIPS,
    TERRAIN_DRAINS,
    TERRAIN_TYREWALLS,
    TERRAIN_CEMENTWALLS,
    TERRAIN_GUARDRAILS,
    TERRAIN_SAND,
    TERRAIN_BUMPY_SAND,
    TERRAIN_DIRT,
    TERRAIN_BUMPY_DIRT,
    TERRAIN_DIRT_ROAD,
    TERRAIN_BUMPY_DIRT_ROAD,
    TERRAIN_PAVEMENT,
    TERRAIN_DIRT_BANK,
    TERRAIN_WOOD,
    TERRAIN_DRY_VERGE,
    TERRAIN_EXIT_RUMBLE_STRIPS,
    TERRAIN_GRASSCRETE,
    TERRAIN_LONG_GRASS,
    TERRAIN_SLOPE_GRASS,
    TERRAIN_COBBLES,
    TERRAIN_SAND_ROAD,
    TERRAIN_BAKED_CLAY,
    TERRAIN_ASTROTURF,
    TERRAIN_SNOWHALF,
    TERRAIN_SNOWFULL,
    TERRAIN_DAMAGED_ROAD1,
    TERRAIN_TRAIN_TRACK_ROAD,
    TERRAIN_BUMPYCOBBLES,
    TERRAIN_ARIES_ONLY,
    TERRAIN_ORION_ONLY,
    TERRAIN_B1RUMBLES,
    TERRAIN_B2RUMBLES,
    TERRAIN_ROUGH_SAND_MEDIUM,
    TERRAIN_ROUGH_SAND_HEAVY,
    TERRAIN_SNOWWALLS,
    TERRAIN_ICE_ROAD,
    TERRAIN_RUNOFF_ROAD,
    TERRAIN_ILLEGAL_STRIP,
    TERRAIN_PAINT_CONCRETE,
    TERRAIN_PAINT_CONCRETE_ILLEGAL,
    TERRAIN_RALLY_TARMAC,
    //-------------
    TERRAIN_MAX,
}

pub enum CRASH_STATES {
    CRASH_DAMAGE_NONE,
    CRASH_DAMAGE_OFFTRACK,
    CRASH_DAMAGE_LARGE_PROP,
    CRASH_DAMAGE_SPINNING,
    CRASH_DAMAGE_ROLLING,
    //-------------
    CRASH_MAX,
}

// (Type#13) ParticipantInfo struct  (to be used with 'mParticipantInfo')
#[derive(Debug)]
#[repr(C)]
pub struct ParticipantInfo {
    pub mIsActive: bool,
    pub mName: [c_char; STRING_LENGTH_MAX],
    pub mWorldPosition: [f32; VECTOR_AXES::VEC_MAX as usize],
    pub mCurrentLapDistance: f32,
    pub mRacePosition: u32,
    pub mLapsCompleted: u32,
    pub mCurrentLap: u32,
    pub mCurrentSector: i32,
}

pub enum DRS_STATES {
    DRS_INSTALLED = (1 << 0),      // Vehicle has DRS capability
    DRS_ZONE_RULES = (1 << 1),     // 1 if DRS uses F1 style rules
    DRS_AVAILABLE_NEXT = (1 << 2), // detection zone was triggered (only applies to f1 style rules)
    DRS_AVAILABLE_NOW = (1 << 3), // detection zone was triggered and we are now in the zone (only applies to f1 style rules)
    DRS_ACTIVE = (1 << 4),        // Wing is in activated state
}

pub enum ERS_DEPLOYMENT_MODES {
    ERS_DEPLOYMENT_MODE_NONE,  // The vehicle does not support deployment modes
    ERS_DEPLOYMENT_MODE_OFF,   // Regen only, no deployment
    ERS_DEPLOYMENT_MODE_BUILD, // Heavy emphasis towards regen
    ERS_DEPLOYMENT_MODE_BALANCED, // Deployment map automatically adjusted to try and maintain target SoC
    ERS_DEPLOYMENT_MODE_ATTACK,   // More aggressive deployment, no target SoC
    ERS_DEPLOYMENT_MODE_QUAL,     // Maximum deployment, no target Soc
}

// (Type#16) YellowFlagState represents current FCY state (to be used with 'mYellowFlagState')
enum YellowFlagState {
    YFS_INVALID = -1,
    YFS_NONE,         // No yellow flag pending on track
    YFS_PENDING,      // Flag has been thrown, but not yet taken by leader
    YFS_PITS_CLOSED,  // Flag taken by leader, pits not yet open
    YFS_PIT_LEAD_LAP, // Those on the lead lap may pit
    YFS_PITS_OPEN,    // Everyone may pit
    YFS_PITS_OPEN2,   // Everyone may pit
    YFS_LAST_LAP,     // On the last caution lap
    YFS_RESUME,       // About to restart (pace car will duck out)
    YFS_RACE_HALT,    // Safety car will lead field into pits
    //-------------
    YFS_MAXIMUM,
}

#[derive(Debug)]
#[repr(C)]
pub struct SharedMemory {
    pub mVersion: u32,
    pub mBuildVersionNumber: u32,
    pub mGameState: u32,
    pub mSessionState: u32,
    pub mRaceState: u32,
    pub mViewedParticipantIndex: i32,
    pub mNumParticipants: i32,
    pub mParticipantInfo: [ParticipantInfo; STORED_PARTICIPANTS_MAX],
    pub mUnfilteredThrottle: f32,
    pub mUnfilteredBrake: f32,
    pub mUnfilteredSteering: f32,
    pub mUnfilteredClutch: f32,
    pub mCarName: [c_char; STRING_LENGTH_MAX],
    pub mCarClassName: [c_char; STRING_LENGTH_MAX],
    pub mLapsInEvent: u32,
    pub mTrackLocation: [c_char; STRING_LENGTH_MAX],
    pub mTrackVariation: [c_char; STRING_LENGTH_MAX],
    pub mTrackLength: f32,
    pub mNumSectors: i32,
    pub mLapInvalidated: bool,
    pub mBestLapTime: f32,
    pub mLastLapTime: f32,
    pub mCurrentTime: f32,
    pub mSplitTimeAhead: f32,
    pub mSplitTimeBehind: f32,
    pub mSplitTime: f32,
    pub mEventTimeRemaining: f32,
    pub mPersonalFastestLapTime: f32,
    pub mWorldFastestLapTime: f32,
    pub mCurrentSector1Time: f32,
    pub mCurrentSector2Time: f32,
    pub mCurrentSector3Time: f32,
    pub mFastestSector1Time: f32,
    pub mFastestSector2Time: f32,
    pub mFastestSector3Time: f32,
    pub mPersonalFastestSector1Time: f32,
    pub mPersonalFastestSector2Time: f32,
    pub mPersonalFastestSector3Time: f32,
    pub mWorldFastestSector1Time: f32,
    pub mWorldFastestSector2Time: f32,
    pub mWorldFastestSector3Time: f32,
    pub mHighestFlagColour: u32,
    pub mHighestFlagReason: u32,
    pub mPitMode: u32,
    pub mPitSchedule: u32,
    pub mCarFlags: u32,
    pub mOilTempCelsius: f32,
    pub mOilPressureKPa: f32,
    pub mWaterTempCelsius: f32,
    pub mWaterPressureKPa: f32,
    pub mFuelPressureKPa: f32,
    pub mFuelLevel: f32,
    pub mFuelCapacity: f32,
    pub mSpeed: f32,
    pub mRpm: f32,
    pub mMaxRPM: f32,
    pub mBrake: f32,
    pub mThrottle: f32,
    pub mClutch: f32,
    pub mSteering: f32,
    pub mGear: i32,
    pub mNumGears: i32,
    pub mOdometerKM: f32,
    pub mAntiLockActive: bool,
    pub mLastOpponentCollisionIndex: f32,
    pub mBoostActive: bool,
    pub mBoostAmount: f32,

    pub mOrientation: [f32; VECTOR_AXES::VEC_MAX as usize],
    pub mLocalVelocity: [f32; VECTOR_AXES::VEC_MAX as usize],
    pub mWorldVelocity: [f32; VECTOR_AXES::VEC_MAX as usize],
    pub mAngularVelocity: [f32; VECTOR_AXES::VEC_MAX as usize],
    pub mLocalAcceleration: [f32; VECTOR_AXES::VEC_MAX as usize],
    pub mWorldAcceleration: [f32; VECTOR_AXES::VEC_MAX as usize],
    pub mExtentsCentre: [f32; VECTOR_AXES::VEC_MAX as usize],

    pub mTyreFlags: [u32; TYRE_STATES::TYRE_MAX as usize],
    pub mTerrain: [u32; TYRE_STATES::TYRE_MAX as usize],
    pub mTyreY: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mTyreRPS: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mTyreSlipSpeed: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mTyreTemp: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mTyreGrip: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mTyreHeightAboveGround: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mTyreLateralStiffness: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mTyreWear: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mBrakeDamage: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mSuspensionDamage: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mBrakeTempCelsius: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mTyreTreadTemp: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mTyreLayerTemp: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mTyreCarcassTemp: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mTyreRimTemp: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mTyreInternalAirTemp: [f32; TYRE_STATES::TYRE_MAX as usize],

    pub mCrashState: u32,
    pub mAeroDamage: f32,
    pub mEngineDamage: f32,

    pub mAmbientTemperature: f32,
    pub mTrackTemperature: f32,
    pub mRainDensity: f32,
    pub mWindSpeed: f32,
    pub mWindDirectionX: f32,
    pub mWindDirectionY: f32,
    pub mCloudBrightness: f32,

    //PCars2 additions start, version 8
    pub mSequenceNumber: u32,

    pub mWheelLocalPositionY: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mSuspensionTravel: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mSuspensionVelocity: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mAirPressure: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mEngineSpeed: f32,
    pub mEngineTorque: f32,
    pub mWings: [f32; 2],
    pub mHandbrake: f32,

    pub mCurrentSector1Times: [f32; STORED_PARTICIPANTS_MAX],
    pub mCurrentSector2Times: [f32; STORED_PARTICIPANTS_MAX],
    pub mCurrentSector3Times: [f32; STORED_PARTICIPANTS_MAX],
    pub mFastestSector1Times: [f32; STORED_PARTICIPANTS_MAX],
    pub mFastestSector2Times: [f32; STORED_PARTICIPANTS_MAX],
    pub mFastestSector3Times: [f32; STORED_PARTICIPANTS_MAX],
    pub mFastestLapTimes: [f32; STORED_PARTICIPANTS_MAX],
    pub mLastLapTimes: [f32; STORED_PARTICIPANTS_MAX],
    pub mLapsInvalidated: [bool; STORED_PARTICIPANTS_MAX],
    pub mRaceStates: [u32; STORED_PARTICIPANTS_MAX],
    pub mPitModes: [u32; STORED_PARTICIPANTS_MAX],
    pub mOrientations: [[f32; VECTOR_AXES::VEC_MAX as usize]; STORED_PARTICIPANTS_MAX],
    pub mSpeeds: [f32; STORED_PARTICIPANTS_MAX],
    pub mCarNames: [[c_char; STRING_LENGTH_MAX]; STORED_PARTICIPANTS_MAX],
    pub mCarClassNames: [[c_char; STRING_LENGTH_MAX]; STORED_PARTICIPANTS_MAX],

    pub mEnforcedPitStopLap: i32,
    pub mTranslatedTrackLocation: [c_char; STRING_LENGTH_MAX],
    pub mTranslatedTrackVariation: [c_char; STRING_LENGTH_MAX],
    pub mBrakeBias: f32,
    pub mTurboBoostPressure: f32,
    pub mTyreCompound: [[c_char; TYRE_COMPOUND_NAME_LENGTH_MAX]; TYRE_STATES::TYRE_MAX as usize],
    pub mPitSchedules: [u32; STORED_PARTICIPANTS_MAX],
    pub mHighestFlagColours: [u32; STORED_PARTICIPANTS_MAX],
    pub mHighestFlagReasons: [u32; STORED_PARTICIPANTS_MAX],
    pub mNationalities: [u32; STORED_PARTICIPANTS_MAX],
    pub mSnowDensity: f32,

    // AMS2 Additions (v10...)
    pub mSessionDuration: f32,
    pub mSessionAdditionalLaps: i32,

    pub mTyreTempLeft: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mTyreTempCenter: [f32; TYRE_STATES::TYRE_MAX as usize],
    pub mTyreTempRight: [f32; TYRE_STATES::TYRE_MAX as usize],

    pub mDrsState: u32,

    pub mRideHeight: [f32; TYRE_STATES::TYRE_MAX as usize],

    pub mJoyPad0: u32,
    pub mDPad: u32,

    pub mAntiLockSetting: i32,
    pub mTractionControlSetting: i32,

    pub mErsDeploymentMode: i32,
    pub mErsAutoModeEnabled: bool,

    pub mClutchTemp: f32,
    pub mClutchWear: f32,
    pub mClutchOverheated: bool,
    pub mClutchSlipping: bool,

    pub mYellowFlagState: i32,
}

pub fn from_slice_u8_to_SharedMemory(slice: &[u8]) -> &SharedMemory {
    let (head, body, _tail) = unsafe { slice.align_to::<SharedMemory>() };
    assert!(head.is_empty(), "Data was not aligned");
    let deserialized_struct = &body[0];
    deserialized_struct
}
