use domain::core::entry::*;

impl EntryCSV {
    pub fn to_entry(&self) -> Entry {
        Entry::new(
            information_access: Some(InformationAccess::new(
                None, // ?)?
                Some(self.global_acces_region),
                Some(self.global_acces_departement),
                None, // ?)?
                Some(self.part_des_familles_monoparentales),
                Some(self.part_des_menages_personne),
                Some(self.service_publics),
                None, // ?)?
            )),
            numeric_interfaces_access: Some(NumericInterfacesAccess::new(
                None, // ?)?
                self.acces_aux_interfaces_numeriques_region,
                self.acces_aux_interfaces_numeriques_departement,
                None, // ?)?
                high_speed_internet_access_percent: Option<f64>,
                mobile_network_availability_percent: Option<f64>,
                percent_of_poor_people: Option<f64>,
                available_median_salary: Option<f64>)
            ),
            administrative_competencies: Option<AdministrativeCompetencies>,
            numeric_competencies: Option<NumericCompetencies>,
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryCSV {
    #[serde(rename(deserialize = "Libdep"))]
    libdep: String,
    #[serde(rename(deserialize = "Libepci"))]
    libepci: String,
    #[serde(rename(deserialize = "Libreg"))]
    libreg: String,
    #[serde(rename(deserialize = "P16 Pop"))]
    p16_pop: String,
    #[serde(rename(deserialize = "SCORE GLOBAL departement 1"))]
    score_global_departement_1: String,
    #[serde(rename(deserialize = "SCORE GLOBAL epci 1"))]
    score_global_epci_1: String,
    #[serde(rename(deserialize = "Calcul 1"))]
    calcul_1: String,
    #[serde(rename(deserialize = "Code Iris"))]
    code_iris: String,
    #[serde(rename(deserialize = "COM"))]
    com: String,
    #[serde(rename(deserialize = "DEP"))]
    dep: String,
    #[serde(rename(deserialize = "Epci"))]
    epci: String,
    #[serde(rename(deserialize = "Geo Point"))]
    geo_point: String,
    #[serde(rename(deserialize = "Geo Shape"))]
    geo_shape: String,
    #[serde(rename(deserialize = "Grd Quart"))]
    grd_quart: String,
    #[serde(rename(deserialize = "Insee Com"))]
    insee_com: String,
    #[serde(rename(deserialize = "Insee Dep"))]
    insee_dep: String,
    #[serde(rename(deserialize = "Insee Reg"))]
    insee_reg: String,
    #[serde(rename(deserialize = "Iris"))]
    iris: String,
    #[serde(rename(deserialize = "Libiris"))]
    libiris: String,
    #[serde(rename(deserialize = "Nom Com"))]
    nom_com: String,
    #[serde(rename(deserialize = "Nom Dep"))]
    nom_dep: String,
    #[serde(rename(deserialize = "Nom Iris"))]
    nom_iris: String,
    #[serde(rename(deserialize = "Nom Reg"))]
    nom_reg: String,
    #[serde(rename(deserialize = "Part des chômeurs (15 – 64 ans)"))]
    pat_chomeurs: Option<String>,
    #[serde(rename(deserialize = "REG"))]
    reg: String,
    #[serde(rename(deserialize = "Services publics / individu"))]
    service_publics: String,
    #[serde(rename(deserialize = "Taux de couverture HD / THD (DSL, câble, FttH)"))]
    taux_couv_hd_thd: Option<String>,
    #[serde(rename(deserialize = "Taux de couverture HD / THD (DSL, câble, FttH) 1"))]
    taux_couv_hd_thd_1: Option<String>,
    #[serde(rename(deserialize = "Taux de couverture mobile (2G)"))]
    taux_couv_mobile: Option<String>,
    #[serde(rename(deserialize = "Taux de pauvreté"))]
    taux_pauvrete: Option<String>,
    #[serde(rename(deserialize = "Type Iris"))]
    type_iris: Option<String>,
    #[serde(rename(deserialize = "4E Trimestre 2016 Données Communales"))]
    quatre_trim_2016_donnees_communales: String,
    #[serde(rename(deserialize = "ACCES A L'INFORMATION departement"))]
    access_information_departement: String,
    #[serde(rename(deserialize = "ACCES A L'INFORMATION departement 1"))]
    acces_information_departement_1: String,
    #[serde(rename(deserialize = "ACCES A L'INFORMATION departement*  "))]
    acces_information_departement: String,
    #[serde(rename(deserialize = "ACCES A L'INFORMATION epci"))]
    acces_information_epci: String,
    #[serde(rename(deserialize = "ACCES A L'INFORMATION epci * "))]
    acces_information_epci_star: String,
    #[serde(rename(deserialize = "ACCES A L'INFORMATION epci 1"))]
    acces_information_epci_1: String,
    #[serde(rename(deserialize = "ACCES A L'INFORMATION region"))]
    acces_information_region: String,
    #[serde(rename(deserialize = "ACCES A L'INFORMATION region * "))]
    acces_information_region_star: String,
    #[serde(rename(deserialize = "ACCES A L'INFORMATION region 1"))]
    acces_information_region_1: String,
    #[serde(rename(deserialize = "ACCÈS AUX INTERFACES NUMERIQUES departement"))]
    acces_aux_interfaces_numeriques_departement: String,
    #[serde(rename(deserialize = "ACCÈS AUX INTERFACES NUMERIQUES departement *  "))]
    acces_aux_interfaces_numeriques_departement_star: String,
    #[serde(rename(deserialize = "ACCÈS AUX INTERFACES NUMERIQUES departement 1"))]
    acces_aux_interfaces_numeriques_departement_1: String,
    #[serde(rename(deserialize = "ACCÈS AUX INTERFACES NUMERIQUES epci"))]
    acces_aux_interfaces_numeriques_epci: String,
    #[serde(rename(deserialize = "ACCÈS AUX INTERFACES NUMERIQUES epci *  "))]
    acces_aux_interfaces_numeriques_epci_star: String,
    #[serde(rename(deserialize = "ACCÈS AUX INTERFACES NUMERIQUES epci 1"))]
    acces_aux_interfaces_numeriques_epci_1: String,
    #[serde(rename(deserialize = "ACCÈS AUX INTERFACES NUMERIQUES region"))]
    acces_aux_interfaces_numeriques_region: String,
    #[serde(rename(deserialize = "ACCÈS AUX INTERFACES NUMERIQUES region *  "))]
    acces_aux_interfaces_numeriques_region_star: String,
    #[serde(rename(deserialize = "ACCÈS AUX INTERFACES NUMERIQUES region 1"))]
    acces_aux_interfaces_numeriques_region_1: String,
    #[serde(rename(deserialize = "C16 Fam"))]
    c_16_fam: String,
    #[serde(rename(deserialize = "C16 Fammono"))]
    c_16_fammono: String,
    #[serde(rename(deserialize = "C16 Men"))]
    c_16_men: String,
    #[serde(rename(deserialize = "C16 Menpseul"))]
    c_16_menpseul: String,
    #[serde(rename(deserialize = "CM Part des familles departement"))]
    cm_part_des_familles_departement: String,
    #[serde(rename(deserialize = "CM Part des familles EPCI"))]
    cm_part_des_familles_epci: String,
    #[serde(rename(deserialize = "CM Part des familles region"))]
    cm_part_des_familles_region: String,
    #[serde(rename(deserialize = "CM Part des ménages departement"))]
    cm_part_des_menages_departement: String,
    #[serde(rename(deserialize = "CM Part des ménages EPCI"))]
    cm_part_des_menages_epci: String,
    #[serde(rename(deserialize = "CM Part des ménages region"))]
    cm_part_des_menages_region: String,
    #[serde(rename(deserialize = "CM Part des non ou peu diplômés departement"))]
    cm_part_des_non_ou_peu_diplomes_departement: String,
    #[serde(rename(deserialize = "CM Part des non ou peu diplômés EPCI"))]
    cm_part_des_non_ou_peu_diplomes_epci: String,
    #[serde(rename(deserialize = "CM Part des non ou peu diplômés region"))]
    cm_part_des_non_ou_peu_diplomes_region: String,
    #[serde(rename(deserialize = "CM Part des personnes âgées de 15 – 29 ans departement"))]
    cm_part_des_personnes_agees_de_15_29_ans_departement: String,
    #[serde(rename(deserialize = "CM Part des personnes âgées de 15 – 29 ans EPCI"))]
    cm_part_des_personnes_agees_de_15_29_ans_epci: String,
    #[serde(rename(deserialize = "CM Part des personnes âgées de 15 – 29 ans region"))]
    cm_part_des_personnes_agees_de_15_29_ans_region: String,
    #[serde(rename(deserialize = "CM Part des personnes âgées de 65 departement"))]
    cm_part_des_personnes_agees_de_65_departement: String,
    #[serde(rename(deserialize = "CM Part des personnes âgées de 65 EPCI"))]
    cm_part_des_personnes_agees_de_65_epci: String,
    #[serde(rename(deserialize = "CM Part des personnes âgées de 65 region"))]
    cm_part_des_personnes_agees_de_65_region: String,
    #[serde(rename(deserialize = "CM revenue median departement"))]
    cm_revenue_median_departement: String,
    #[serde(rename(deserialize = "CM revenue median EPCI"))]
    cm_revenue_median_epci: String,
    #[serde(rename(deserialize = "CM revenue median region"))]
    cm_revenue_median_region: String,
    #[serde(rename(deserialize = "COMPETENCES ADMINISTATIVES departement"))]
    competences_administatives_departement: String,
    #[serde(rename(deserialize = "COMPETENCES ADMINISTATIVES departement  *"))]
    competences_administatives_departement_star: String,
    #[serde(rename(deserialize = "COMPETENCES ADMINISTATIVES departement 1"))]
    competences_administatives_departement_1: String,
    #[serde(rename(deserialize = "COMPETENCES ADMINISTATIVES epci"))]
    competences_administatives_epci: String,
    #[serde(rename(deserialize = "COMPETENCES ADMINISTATIVES epci  * "))]
    competences_administatives_epci_star: String,
    #[serde(rename(deserialize = "COMPETENCES ADMINISTATIVES epci 1"))]
    competences_administatives_epci_1: String,
    #[serde(rename(deserialize = "COMPETENCES ADMINISTATIVES region"))]
    competences_administatives_region: String,
    #[serde(rename(deserialize = "COMPETENCES ADMINISTATIVES region * "))]
    competences_administatives_region_star: String,
    #[serde(rename(deserialize = "COMPETENCES ADMINISTATIVES region 1"))]
    competences_administatives_region_1: String,
    #[serde(rename(deserialize = "COMPÉTENCES NUMÉRIQUES / SCOLAIRES departement"))]
    competences_numeriques_scolaires_departement: String,
    #[serde(rename(deserialize = "COMPÉTENCES NUMÉRIQUES / SCOLAIRES departement * "))]
    competences_numeriques_scolaires_departement_star: String,
    #[serde(rename(deserialize = "COMPÉTENCES NUMÉRIQUES / SCOLAIRES departement 1"))]
    competences_numeriques_scolaires_departement_1: String,
    #[serde(rename(deserialize = "COMPÉTENCES NUMÉRIQUES / SCOLAIRES epci"))]
    competences_numeriques_scolaires_epci: String,
    #[serde(rename(deserialize = "COMPÉTENCES NUMÉRIQUES / SCOLAIRES epci * "))]
    competences_numeriques_scolaires_epci_star: String,
    #[serde(rename(deserialize = "COMPÉTENCES NUMÉRIQUES / SCOLAIRES epci 1"))]
    competences_numeriques_scolaires_epci_1: String,
    #[serde(rename(deserialize = "COMPÉTENCES NUMÉRIQUES / SCOLAIRES region"))]
    competences_numeriques_scolaires_region: String,
    #[serde(rename(deserialize = "COMPÉTENCES NUMÉRIQUES / SCOLAIRES region * "))]
    competences_numeriques_scolaires_region_star: String,
    #[serde(rename(deserialize = "COMPÉTENCES NUMÉRIQUES / SCOLAIRES region 1"))]
    competences_numeriques_scolaires_region_1: String,
    #[serde(rename(deserialize = "Dec Med15"))]
    dec_med_15: String,
    #[serde(rename(deserialize = "F27"))]
    f_27: String,
    #[serde(rename(deserialize = "F29"))]
    f_29: String,
    #[serde(rename(deserialize = "GLOBAL ACCES departement"))]
    global_acces_departement: String,
    #[serde(rename(deserialize = "GLOBAL ACCES departement  *"))]
    global_acces_departement_star: String,
    #[serde(rename(deserialize = "GLOBAL ACCES departement 1"))]
    global_acces_departement_1: String,
    #[serde(rename(deserialize = "GLOBAL ACCES epci"))]
    global_acces_epci: String,
    #[serde(rename(deserialize = "GLOBAL ACCES epci *"))]
    global_acces_epci_star: String,
    #[serde(rename(deserialize = "GLOBAL ACCES epci 1"))]
    global_acces_epci_1: String,
    #[serde(rename(deserialize = "GLOBAL ACCES region"))]
    global_acces_region: String,
    #[serde(rename(deserialize = "GLOBAL ACCES region * "))]
    global_acces_region_star: String,
    #[serde(rename(deserialize = "GLOBAL ACCES region 1"))]
    global_acces_region_1: String,
    #[serde(rename(deserialize = "GLOBAL COMPETENCES  departement"))]
    global_competences_departement: String,
    #[serde(rename(deserialize = "GLOBAL COMPETENCES  departement  * "))]
    global_competences_departement_star: String,
    #[serde(rename(deserialize = "GLOBAL COMPETENCES  departement 1"))]
    global_competences_departement_1: String,
    #[serde(rename(deserialize = "GLOBAL COMPETENCES epci"))]
    global_competences_epci: String,
    #[serde(rename(deserialize = "GLOBAL COMPETENCES epci *"))]
    global_competences_epci_star: String,
    #[serde(rename(deserialize = "GLOBAL COMPETENCES epci 1"))]
    global_competences_epci_1: String,
    #[serde(rename(deserialize = "GLOBAL COMPETENCES region"))]
    global_competences_region: String,
    #[serde(rename(deserialize = "GLOBAL COMPETENCES region *  "))]
    global_competences_region_star: String,
    #[serde(rename(deserialize = "GLOBAL COMPETENCES region 1"))]
    global_competences_region_1: String,
    #[serde(rename(deserialize = "Géométrie"))]
    geometrie: String,
    #[serde(rename(deserialize = "MED15 données communales"))]
    med_15_donnees_communales: String,
    #[serde(rename(deserialize = "Nombre d'enregistrements"))]
    nombre_enregistrements: String,
    #[serde(rename(deserialize = "P16 Nscol15P"))]
    p_16_nscol_15_p: String,
    #[serde(rename(deserialize = "P16 Nscol15P Diplmin"))]
    p_16_nscol_15_p_diplmin: String,
    #[serde(rename(deserialize = "P16 Pop Imm"))]
    p_16_pop_imm: String,
    #[serde(rename(deserialize = "P16 Pop1529"))]
    p_16_pop_1529: String,
    #[serde(rename(deserialize = "P16 Pop65P"))]
    p_16_pop_65_p: String,
    #[serde(rename(deserialize = "Part des familles monoparentales"))]
    part_des_familles_monoparentales: String,
    #[serde(rename(deserialize = "Part des ménages d'une personne"))]
    part_des_menages_personne: String,
    #[serde(rename(
        deserialize = "Part des non ou peu diplômés dans la population non scolarisée de 15 ans ou plus"
    ))]
    part_des_non_peu_diplomes_population_non_scolarisee_15_ans_plus: String,
    #[serde(rename(deserialize = "Part des personnes âgées de 15 – 29 ans"))]
    part_des_personnes_agees_de_15_29_ans: String,
    #[serde(rename(deserialize = "Part des personnes âgées de 65 ans / +"))]
    part_des_personnes_agees_de_65_ans_plus: String,
    #[serde(rename(deserialize = "Revenus médian disponible (si communale)"))]
    revenus_median_disponible_si_communale: String,
    #[serde(rename(deserialize = "Revenus médian disponible (si infra-communale)"))]
    revenus_median_disponible_si_infra_communale: String,
    #[serde(rename(deserialize = "SCORE GLOBAL departement"))]
    score_global_departement: String,
    #[serde(rename(deserialize = "SCORE GLOBAL departement *"))]
    score_global_departement_star: String,
    #[serde(rename(deserialize = "SCORE GLOBAL epci"))]
    score_global_epci: String,
    #[serde(rename(deserialize = "SCORE GLOBAL epci * "))]
    score_global_epci_star: String,
    #[serde(rename(deserialize = "SCORE GLOBAL region"))]
    score_global_region: String,
    #[serde(rename(deserialize = "SCORE GLOBAL region * "))]
    score_global_region_star: String,
    #[serde(rename(deserialize = "SEUILS Part des familles departement"))]
    seuils_part_des_familles_departement: String,
    #[serde(rename(deserialize = "SEUILS Part des familles EPCI"))]
    seuils_part_des_familles_epci: String,
    #[serde(rename(deserialize = "SEUILS Part des familles region"))]
    seuils_part_des_familles_region: String,
    #[serde(rename(deserialize = "SEUILS Part des ménages departement"))]
    seuils_part_des_menages_departement: String,
    #[serde(rename(deserialize = "SEUILS Part des ménages EPCI"))]
    seuils_part_des_menages_epci: String,
    #[serde(rename(deserialize = "SEUILS Part des ménages region"))]
    seuils_part_des_menages_region: String,
    #[serde(rename(deserialize = "SEUILS Part des non ou peu diplômés departement"))]
    seuils_part_des_non_ou_peu_diplomes_departement: String,
    #[serde(rename(deserialize = "SEUILS Part des non ou peu diplômés EPCI"))]
    seuils_part_des_non_ou_peu_diplomes_epci: String,
    #[serde(rename(deserialize = "SEUILS Part des non ou peu diplômés region"))]
    seuils_part_des_non_ou_peu_diplomes_region: String,
    #[serde(rename(deserialize = "SEUILS Part des personnes âgées de 15 – 29 ans departement"))]
    seuils_part_des_personnes_agees_de_15_29_ans_departement: String,
    #[serde(rename(deserialize = "SEUILS Part des personnes âgées de 15 – 29 ans EPCI"))]
    seuils_part_des_personnes_agees_de_15_29_ans_epci: String,
    #[serde(rename(deserialize = "SEUILS Part des personnes âgées de 15 – 29 ans region"))]
    seuils_part_des_personnes_agees_de_15_29_ans_region: String,
    #[serde(rename(deserialize = "SEUILS Part des personnes âgées de 65 departement"))]
    seuils_part_des_personnes_agees_de_65_departement: String,
    #[serde(rename(deserialize = "SEUILS Part des personnes âgées de 65 EPCI"))]
    seuils_part_des_personnes_agees_de_65_epci: String,
    #[serde(rename(deserialize = "SEUILS Part des personnes âgées de 65 region"))]
    seuils_part_des_personnes_agees_de_65_region: String,
    #[serde(rename(deserialize = "SEUILS revenue median departement"))]
    seuils_revenue_median_departement: String,
    #[serde(rename(deserialize = "SEUILS revenue median EPCI"))]
    seuils_revenue_median_epci: String,
    #[serde(rename(deserialize = "SEUILS revenue median region"))]
    seuils_revenue_median_region: String,
}
