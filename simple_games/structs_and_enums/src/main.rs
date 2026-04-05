struct executive_team {
    name: String,
    profession: String,
    department: String,
    age: u32,
    position: String
}

fn main() {

    let member_1 = executive_team {
        name: String::from("Onaho Pascal"),
        position: String::from("NCMH Executive Director"),
        profession: String::from("Pharmacist, Computational Biologist"),
        age: 27,
        department: String::from("Computational Biology/Bioinformatics")
    };

    let member_2 = executive_team {
        name: String::new("Martins");
        position: String::new("NCMH CADD Director");
        profession: String::new("Pharmacist, Computational Chemist"),
        age: 28,
        department: String::new("Computational Chemistry/CADD"),
    }
}
