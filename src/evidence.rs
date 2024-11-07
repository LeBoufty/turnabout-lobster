// Struct for a piece of evidence that is stored in the court record
struct Evidence {
    fullname: String,       // Full name of the piece of evidence i.e. "Revolver"
    alias: String,          // Shortened name for terminal use i.e. "rev"
    description: String,    // Description of the piece i.e. "The weapon of the crime. Has Joe's fingerprints."
    weight: u8              // Location of the piece in the court record, smaller goes first.
}