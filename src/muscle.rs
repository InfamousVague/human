use std::vec;

pub struct DetailedMuscle {
    pub name: String,
    pub common_name: Option<String>,
    pub description: Option<String>,
    pub location: Option<MuscleLocation>,
    pub action: Option<MuscleAction>,
    pub image: Option<String>
}

impl Default for DetailedMuscle {
    fn default() -> DetailedMuscle {
        DetailedMuscle {
            name: String::from("Un-Specified Muscle"),
            common_name: None,
            description: None,
            location: None,
            action: None,
            image: None
        }
    }
}

pub enum MuscleAction {
    Abduction,
    Adduction,
    Elevation,
    Flexion,
    Inversion,
    Eversion,
    Extension,
    Plantarflexion,
    Dorsiflexion,
    Pronation,
    Supination,
    Protraction,
    Retraction
}

pub enum ContractionType {
    Concentric,
    Eccentric,
    Isometric
}

pub enum MuscleLocation {
    Head,
    Neck,
    Back,
    UpperLimb,
    Forearm,
    Hand,
    Torso,
    Abdomen,
    Pelvis,
    Posterior,
    LowerLimb,
    Leg,
    Foot
}

impl MuscleLocation {
    pub fn get_muscles(&self) -> Vec<Muscle> {
        match self {
            MuscleLocation::Head => vec![
                Muscle::Occipitalis,
                Muscle::Frontalis,
                Muscle::OrbicularisOculi,
                Muscle::CorrugatorSupercilii,
                Muscle::DepressorSupercilii,
                Muscle::LevatorPalpebraeSuperioris,
                Muscle::SuperiorTarsal,
                Muscle::RectusSuperior,
                Muscle::RectusInferior,
                Muscle::RectusMedialis,
                Muscle::RectusLateralis,
                Muscle::ObliquusSuperior,
                Muscle::ObliqussInferior,
                Muscle::Temporoparietalis,
                Muscle::AuricularisAnterior,
                Muscle::AuricularisSuperior,
                Muscle::AuricularisPosterior,
                Muscle::HelicisMajor,
                Muscle::HelicisMinor,
                Muscle::Tragicus,
                Muscle::Antitragicus,
                Muscle::TransversusAuriculae,
                Muscle::ObliqueAuricle,
                Muscle::Stapedius,
                Muscle::TensorTympani,
                Muscle::Procerus,
                Muscle::DepressorSeptiNasi,
                Muscle::Mentalis,
                Muscle::Bbucinator,
                Muscle::OrbicularisOris,
                Muscle::Risorius,
                Muscle::ZygomaticusMajor,
                Muscle::ZygomaticusMinor,
                Muscle::MasseterSuperficialPart,
                Muscle::MasseterDeepPart,
                Muscle::Temporalis,
                Muscle::PterygoidMedial,
                Muscle::PterygoidLateral,
                Muscle::GenioglossusInferior,
                Muscle::GenioglossusSuperior,
                Muscle::Hyoglossus,
                Muscle::Chondroglossus,
                Muscle::Styloglossus,
                Muscle::Palatoglossus,
                Muscle::TransversusLinguae,
                Muscle::VerticalisLinguae,
                Muscle::SuperiorLongitudinalLingual,
                Muscle::InferiorLongitudinalLingual,
                Muscle::TensorVeliPalatini,
                Muscle::LevatorVeliPalatini,
                Muscle::Palatopharyngeus,
                Muscle::Uvulae,
                Muscle::Stylopharyngeus,
                Muscle::Salpingopharyngeus,
                Muscle::PharyngealConstrictor,
                Muscle::Cricothyroid,
                Muscle::ArytenoidTransverse,
                Muscle::ArytenoidOblique,
                Muscle::Vocalis,
                Muscle::Thyroarytenoid,
                Muscle::CricoarytenoidPosterior,
                Muscle::CricoarytenoidLateral
            ],
            MuscleLocation::Neck => vec![
                Muscle::Platysma,
                Muscle::Sternocleidomastoid,
                Muscle::Digastric,
                Muscle::Stylohyoid,
                Muscle::Mylohyoid,
                Muscle::Geniohyoid,
                Muscle::Sternohyoid,
                Muscle::Omohyoid,
                Muscle::LongusColli,
                Muscle::LongusCapitis,
                Muscle::RectusCapitisAnterior,
                Muscle::RectusCapitisLateralis
            ],
            MuscleLocation::Back => vec![
                Muscle::IliocostalisLumborum,
                Muscle::IliocostalisThoracis,
                Muscle::LongissimusCapitis,
                Muscle::LongissimusColli,
                Muscle::SpinalisCapitis,
                Muscle::SpinalisColli,
                Muscle::SpinalisThoracis,
                Muscle::LatissimusDorsi,
                Muscle::SemispinalisThoracis,
                Muscle::SemispinalisColli,
                Muscle::SemispinalisCapitis
            ],
            MuscleLocation::UpperLimb => vec![
                Muscle::LatissimusDorsi,
                Muscle::Trapezius,
                Muscle::RhomboidMajor,
                Muscle::RhomboidMinor,
                Muscle::Sternalis,
                Muscle::PectoralisMajor,
                Muscle::PectoralisMinor,
                Muscle::Subclavius,
                Muscle::SerratusAnterior,
                Muscle::Deltoid,
                Muscle::TeresMajor,
                Muscle::TeresMinor,
                Muscle::Supraspinatus,
                Muscle::Infraspinatus,
                Muscle::Subscapularis,
                Muscle::Coracobrachialis,
                Muscle::BicepsBrachii,
                Muscle::Brachialis,
                Muscle::TricepsBrachii,
                Muscle::ArticularisCubiti,
                Muscle::Anconeus,
                Muscle::PronatorTeres,
                Muscle::FlexorCarpiRadialis,
                Muscle::PalmarisLongus,
                Muscle::FlexorCarpiUlnaris,
                Muscle::FlexorDigitorumSuperficialis,
                Muscle::PronatorQuadratus,
                Muscle::FlexorDigitorumProfundus,
                Muscle::FlexorPollicisLongus,
                Muscle::ExtensorDigitorum,
                Muscle::ExtensorDigitiMinimi,
                Muscle::ExtensorCarpiUlnaris,
                Muscle::Brachioradialis,
                Muscle::ExtensorCarpiRadialisLongus,
                Muscle::ExtensorCarpiRadialisBrevis,
                Muscle::ExtensorIndicis,
                Muscle::AbductorPollicisLongus,
                Muscle::ExtensorPollicisBrevis,
                Muscle::ExtensorPollicisLongus,
                Muscle::OpponensPollicis,
                Muscle::FlexorPollicisBrevis,
                Muscle::AbductorPollicisBrevis,
                Muscle::AdductorPollicis,
                Muscle::PalmarisBrevis,
                Muscle::AbductorDigitiMinimi,
                Muscle::FlexorDigitiMinimi,
                Muscle::OpponensDigitiMinimi,
                Muscle::Lumbricals,
                Muscle::DorsalInterossei,
                Muscle::PalmarInterossei,
                Muscle::PollicalPalmarInterosseus
            ],
            MuscleLocation::Forearm => vec![
                Muscle::PronatorTeres,
                Muscle::FlexorCarpiRadialis,
                Muscle::PalmarisLongus,
                Muscle::FlexorCarpiUlnaris,
                Muscle::FlexorDigitorumSuperficialis,
                Muscle::PronatorQuadratus,
                Muscle::FlexorDigitorumProfundus,
                Muscle::FlexorPollicisLongus,
                Muscle::ExtensorDigitorum,
                Muscle::ExtensorDigitiMinimi,
                Muscle::ExtensorCarpiUlnaris,
                Muscle::Brachioradialis,
                Muscle::ExtensorCarpiRadialisLongus,
                Muscle::ExtensorCarpiRadialisBrevis,
                Muscle::ExtensorIndicis,
                Muscle::AbductorPollicisLongus,
                Muscle::ExtensorPollicisBrevis,
                Muscle::ExtensorPollicisLongus,
                Muscle::OpponensPollicis,
                Muscle::FlexorPollicisBrevis,
                Muscle::AbductorPollicisBrevis,
                Muscle::AdductorPollicis,
                Muscle::PalmarisBrevis,
                Muscle::AbductorDigitiMinimi,
                Muscle::FlexorDigitiMinimi,
                Muscle::OpponensDigitiMinimi,
                Muscle::Lumbricals,
                Muscle::DorsalInterossei,
                Muscle::PalmarInterossei,
                Muscle::PollicalPalmarInterosseus
            ],
            MuscleLocation::Hand => vec![
                Muscle::FlexorPollicisBrevis,
                Muscle::AbductorPollicisBrevis,
                Muscle::AdductorPollicis,
                Muscle::PalmarisBrevis,
                Muscle::AbductorDigitiMinimi,
                Muscle::FlexorDigitiMinimi,
                Muscle::OpponensDigitiMinimi,
                Muscle::Lumbricals,
                Muscle::DorsalInterossei,
                Muscle::PalmarInterossei,
                Muscle::PollicalPalmarInterosseus
            ],
            MuscleLocation::Torso => vec![
                Muscle::PsoasMajor,
                Muscle::PsoasMinor,
                Muscle::Iliacus,
                Muscle::TensorFasciaeLatae,
                Muscle::GluteusMaximus,
                Muscle::GluteusMedius,
                Muscle::GluteusMinimus,
                Muscle::Piriformis,
                Muscle::ObturatorExternus,
                Muscle::SuperiorGemellus,
                Muscle::ObturatorInternus,
                Muscle::InferiorGemellus,
                Muscle::QuadratusFemoris,
                Muscle::ArticularisGenus,
                Muscle::Sartorius,
                Muscle::RectusFemoris,
                Muscle::VastusLateralis,
                Muscle::VastusMedialis,
                Muscle::VastusIntermedius,
                Muscle::BicepsFemoris,
                Muscle::Semitendinosus,
                Muscle::Semimembranosus,
                Muscle::Gracilis,
                Muscle::Pectineus,
                Muscle::AdductorBrevis,
                Muscle::AdductorLongus,
                Muscle::AdductorMagnus,
                Muscle::AdductorMinimus
            ],
            MuscleLocation::Abdomen => vec![
                Muscle::RectusAbdominis,
                Muscle::ExternalAbdominalOblique,
                Muscle::InternalAbdominalOblique,
                Muscle::Cremaster,
                Muscle::TransversusAbdominis,
                Muscle::Pyramidalis,
                Muscle::QuadratusLumborum
            ],
            MuscleLocation::Pelvis => vec![
                Muscle::Iliococcygeus,
                Muscle::Pubococcygeus,
                Muscle::Puboanalis,
                Muscle::ExternalAnalSphincter,
                Muscle::SuperficialTransversePerineal,
                Muscle::Bulbospongiosus,
                Muscle::Ischiocavernosus,
                Muscle::DeepTransversePerineal,
                Muscle::CompressorUrethrae,
                Muscle::SphincterUrethrovaginalis,
                Muscle::ExternalUrethralSphincter,
                Muscle::Coccygeus
            ],
            MuscleLocation::Posterior => vec![
                Muscle::SpleniusCapitis,
                Muscle::SpleniusColli,
                Muscle::ExternalIntercostal,
                Muscle::InternalIntercostal,
                Muscle::InnermostIntercostal,
                Muscle::Subcostales,
                Muscle::TransversusThoracis,
                Muscle::LevatoresCostarum,
                Muscle::SerratusPosteriorInferior,
                Muscle::SerratusPosteriorSuperior,
                Muscle::Hemidiaphragm
            ],
            MuscleLocation::LowerLimb => vec![
                Muscle::PsoasMajor,
                Muscle::PsoasMinor,
                Muscle::Iliacus,
                Muscle::TensorFasciaeLatae,
                Muscle::GluteusMaximus,
                Muscle::GluteusMedius,
                Muscle::GluteusMinimus,
                Muscle::Piriformis,
                Muscle::ObturatorExternus,
                Muscle::SuperiorGemellus,
                Muscle::ObturatorInternus,
                Muscle::InferiorGemellus,
                Muscle::QuadratusFemoris,
                Muscle::ArticularisGenus,
                Muscle::Sartorius,
                Muscle::RectusFemoris,
                Muscle::VastusLateralis,
                Muscle::VastusMedialis,
                Muscle::VastusIntermedius,
                Muscle::BicepsFemoris,
                Muscle::Semitendinosus,
                Muscle::Semimembranosus,
                Muscle::Gracilis,
                Muscle::Pectineus,
                Muscle::AdductorBrevis,
                Muscle::AdductorLongus,
                Muscle::AdductorMagnus,
                Muscle::AdductorMinimus,
                Muscle::TibialisAnterior,
                Muscle::ExtensorHallucisLongus,
                Muscle::ExtensorDigitorumLongus,
                Muscle::FibularisTertius,
                Muscle::Gastrocnemius,
                Muscle::Soleus,
                Muscle::Plantaris,
                Muscle::Popliteus,
                Muscle::FlexorHallucisLongus,
                Muscle::FlexorDigitorumLongus,
                Muscle::TibialisPosterior,
                Muscle::FibularisLongus,
                Muscle::FibularisBrevis,
                Muscle::ExtensorDigitorumBrevis,
                Muscle::ExtensorHallucisBrevis,
                Muscle::AbductorHallucis,
                Muscle::FlexorDigitorumBrevis,
                Muscle::QuadratusPlantae,
                Muscle::FlexorHallucisBrevis,
                Muscle::AdductorHallucis,
                Muscle::FlexorDigitiMinimiBrevis,
                Muscle::PlantarInterossei
            ],
            MuscleLocation::Leg => vec![
                Muscle::TibialisAnterior,
                Muscle::ExtensorHallucisLongus,
                Muscle::ExtensorDigitorumLongus,
                Muscle::FibularisTertius,
                Muscle::Gastrocnemius,
                Muscle::Soleus,
                Muscle::Plantaris,
                Muscle::Popliteus,
                Muscle::FlexorHallucisLongus,
                Muscle::FlexorDigitorumLongus,
                Muscle::TibialisPosterior,
                Muscle::FibularisLongus,
                Muscle::FibularisBrevis,
                Muscle::ExtensorDigitorumBrevis,
                Muscle::ExtensorHallucisBrevis,
                Muscle::AbductorHallucis,
                Muscle::FlexorDigitorumBrevis,
                Muscle::QuadratusPlantae,
                Muscle::FlexorHallucisBrevis,
                Muscle::AdductorHallucis,
                Muscle::FlexorDigitiMinimiBrevis,
                Muscle::PlantarInterossei
            ],
            MuscleLocation::Foot => vec![
                Muscle::FlexorHallucisBrevis,
                Muscle::AdductorHallucis,
                Muscle::FlexorDigitiMinimiBrevis,
                Muscle::PlantarInterossei
            ]
        }
    }
}

pub enum Muscle {
    Occipitalis,
    Frontalis,
    OrbicularisOculi,
    CorrugatorSupercilii,
    DepressorSupercilii,
    LevatorPalpebraeSuperioris,
    SuperiorTarsal,
    RectusSuperior,
    RectusInferior,
    RectusMedialis,
    RectusLateralis,
    ObliquusSuperior,
    ObliqussInferior,
    Temporoparietalis,
    AuricularisAnterior,
    AuricularisSuperior,
    AuricularisPosterior,
    HelicisMajor,
    HelicisMinor,
    Tragicus,
    Antitragicus,
    TransversusAuriculae,
    ObliqueAuricle,
    Stapedius,
    TensorTympani,
    Procerus,
    DepressorSeptiNasi,
    Mentalis,
    Bbucinator,
    OrbicularisOris,
    Risorius,
    ZygomaticusMajor,
    ZygomaticusMinor,
    MasseterSuperficialPart,
    MasseterDeepPart,
    Temporalis,
    PterygoidMedial,
    PterygoidLateral,
    GenioglossusInferior,
    GenioglossusSuperior,
    Hyoglossus,
    Chondroglossus,
    Styloglossus,
    Palatoglossus,
    TransversusLinguae,
    VerticalisLinguae,
    SuperiorLongitudinalLingual,
    InferiorLongitudinalLingual,
    TensorVeliPalatini,
    LevatorVeliPalatini,
    Palatopharyngeus,
    Uvulae,
    Stylopharyngeus,
    Salpingopharyngeus,
    PharyngealConstrictor,
    Cricothyroid,
    ArytenoidTransverse,
    ArytenoidOblique,
    Vocalis,
    Thyroarytenoid,
    CricoarytenoidPosterior,
    CricoarytenoidLateral,
    Platysma,
    Sternocleidomastoid,
    Digastric,
    Stylohyoid,
    Mylohyoid,
    Geniohyoid,
    Sternohyoid,
    Omohyoid,
    LongusColli,
    LongusCapitis,
    RectusCapitisAnterior,
    RectusCapitisLateralis,
    ScalenusAnterior,
    ScalenusMedius,
    ScalenusPosterior,
    ScalenusMinimus,
    LevatorScapulae,
    ObliquusCapitisSuperior,
    ObliquusCapitisInferior,
    RectusCapitisPosteriorMinor,
    RectusCapitisPosteriorMajor,
    RectusAbdominis,
    ExternalAbdominalOblique,
    InternalAbdominalOblique,
    Cremaster,
    TransversusAbdominis,
    Pyramidalis,
    QuadratusLumborum,
    IliocostalisLumborum,
    IliocostalisThoracis,
    LongissimusCapitis,
    LongissimusColli,
    SpinalisCapitis,
    SpinalisColli,
    SpinalisThoracis,
    LatissimusDorsi,
    SemispinalisThoracis,
    SemispinalisColli,
    SemispinalisCapitis,
    MultifidusLumborum,
    MultifidusThoracis,
    MultifidusColli,
    Rotatores,
    RotatoresLumborum,
    RotatoresColli,
    InterspinalesLumborum,
    Intertransversarii,
    SpleniusCapitis,
    SpleniusColli,
    ExternalIntercostal,
    InternalIntercostal,
    InnermostIntercostal,
    Subcostales,
    TransversusThoracis,
    LevatoresCostarum,
    SerratusPosteriorInferior,
    SerratusPosteriorSuperior,
    Hemidiaphragm,
    Coccygeus,
    Iliococcygeus,
    Pubococcygeus,
    Puboanalis,
    ExternalAnalSphincter,
    SuperficialTransversePerineal,
    Bulbospongiosus,
    Ischiocavernosus,
    DeepTransversePerineal,
    CompressorUrethrae,
    SphincterUrethrovaginalis,
    ExternalUrethralSphincter,
    Trapezius,
    RhomboidMajor,
    RhomboidMinor,
    Sternalis,
    PectoralisMajor,
    PectoralisMinor,
    Subclavius,
    SerratusAnterior,
    Deltoid,
    TeresMajor,
    TeresMinor,
    Supraspinatus,
    Infraspinatus,
    Subscapularis,
    Coracobrachialis,
    BicepsBrachii,
    Brachialis,
    TricepsBrachii,
    ArticularisCubiti,
    Anconeus,
    PronatorTeres,
    FlexorCarpiRadialis,
    PalmarisLongus,
    FlexorCarpiUlnaris,
    FlexorDigitorumSuperficialis,
    PronatorQuadratus,
    FlexorDigitorumProfundus,
    FlexorPollicisLongus,
    ExtensorDigitorum,
    ExtensorDigitiMinimi,
    ExtensorCarpiUlnaris,
    Brachioradialis,
    ExtensorCarpiRadialisLongus,
    ExtensorCarpiRadialisBrevis,
    ExtensorIndicis,
    AbductorPollicisLongus,
    ExtensorPollicisBrevis,
    ExtensorPollicisLongus,
    OpponensPollicis,
    FlexorPollicisBrevis,
    AbductorPollicisBrevis,
    AdductorPollicis,
    PalmarisBrevis,
    AbductorDigitiMinimi,
    FlexorDigitiMinimi,
    OpponensDigitiMinimi,
    Lumbricals,
    DorsalInterossei,
    PalmarInterossei,
    PollicalPalmarInterosseus,
    PsoasMajor,
    PsoasMinor,
    Iliacus,
    TensorFasciaeLatae,
    GluteusMaximus,
    GluteusMedius,
    GluteusMinimus,
    Piriformis,
    ObturatorExternus,
    SuperiorGemellus,
    ObturatorInternus,
    InferiorGemellus,
    QuadratusFemoris,
    ArticularisGenus,
    Sartorius,
    RectusFemoris,
    VastusLateralis,
    VastusMedialis,
    VastusIntermedius,
    BicepsFemoris,
    Semitendinosus,
    Semimembranosus,
    Gracilis,
    Pectineus,
    AdductorBrevis,
    AdductorLongus,
    AdductorMagnus,
    AdductorMinimus,
    TibialisAnterior,
    ExtensorHallucisLongus,
    ExtensorDigitorumLongus,
    FibularisTertius,
    Gastrocnemius,
    Soleus,
    Plantaris,
    Popliteus,
    FlexorHallucisLongus,
    FlexorDigitorumLongus,
    TibialisPosterior,
    FibularisLongus,
    FibularisBrevis,
    ExtensorDigitorumBrevis,
    ExtensorHallucisBrevis,
    AbductorHallucis,
    FlexorDigitorumBrevis,
    QuadratusPlantae,
    FlexorHallucisBrevis,
    AdductorHallucis,
    FlexorDigitiMinimiBrevis,
    PlantarInterossei
}