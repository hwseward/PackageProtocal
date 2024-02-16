
// Derive traits for formating string

#[derive(Clone, Debug)]

// Package Type with 3 different types
pub enum Package {

    //Command type for command packages
    // Command(String::from("shutdown now"))
    Command(String),
    // Doc type for sending files
    // Doc(String::from(""))
    Doc(String),
    // Msg type for sending string
    // Msg(String::from("What's Up"))
    Msg(String),

}

// Byte to show end of package
pub const END_BYTE: u8 = 017;
// Byte to show string type
pub const STR_BYTE: u8 = 02;
// Byte to show command type
pub const COM_BYTE: u8 = 04;
// Byte to show File type
pub const FILE_BYTE: u8 = 05;

// Implementation of enum Package
impl Package {
    
    /// Returns new command Package
    ///
    /// Parameters String for command to be ran
    ///
    /// Example
    /// '''
    /// let pack = Package::new_command(String::from("shutdown now"));
    /// '''
    pub fn new_command(command: String) -> Package {
    
        // returns command variant of package 
        Package::Command(command)

    }

    /// Returns new File Package
    ///
    /// Parameters File for file to be sent
    ///
    /// Example
    /// '''
    /// let pack = Package::new_File(File::create("ex.txt"));
    /// '''
    pub fn new_file(doc: String) -> Package {

        //returns new package type File
        Package::Doc(doc)

    }


    /// Returns new Msg Package
    ///
    /// Parameters String for the message to be sent
    ///
    /// Example
    /// ```
    /// let pack = Package::new_msg(String::from("Hello"));
    /// ```

    pub fn new_msg(msg: String) -> Package {

        // returns new package type msg
        Package::Msg(msg)
    }

    /// Returns new Package from received array and finds type
    ///
    /// Parameters u8[] of package received
    ///
    /// Example
    /// ```
    /// let msg = Package::new_msg(String::from("Hello"));
    /// let msg = msg.to_vec()
    /// let pack = Package::new(&msg);
    /// ```
    pub fn new(pack: &[u8]) -> Package {

        // Matchs aginst 1st byte to find package type
        match pack[0] {

            // Checks for Command byte
            COM_BYTE => {

                // removes the 1st and last byte
                let t = String::from_utf8(pack[1..pack.len()-1].to_vec()).unwrap();
                // returns package Command with bytes removed
                Package::Command(t)

            },

            // Checks for File Byte
            FILE_BYTE => {

                // removes 1st and last byte
                let t = String::from_utf8(pack[1..pack.len()-1].to_vec()).unwrap();
                // returns Package File with bytes removed
                Package::Doc(t)

            },

            // Checks for Msg Byte
            STR_BYTE => {

                // Removes 1st and last Byte
                let t = String::from_utf8(pack[1..pack.len()-1].to_vec()).unwrap();
                // returns Package msg with bytes removed
                Package::Msg(t)

            },
            // Error if Invaild Byte
            _ => panic!("Invaild Byte")

        }
    }

    /// Returns Package converted to Vec<u8> with transfer bytes added
    /// 
    /// Example
    /// ```
    /// let pack = Package::new_msg(String::new("Hello"));
    /// let data = pack.to_vec();
    /// ```
    pub fn to_vec(self) -> Vec<u8> {


        // Checks what variant the package is
        match self {


            // for command Package
            Package::Command(com) =>  {
                // Converts data to bytes
                let mut vec = com.as_bytes().to_vec();
                // Place the starting byte for Command type
                vec.insert(0 ,self::COM_BYTE);
                // Places the end Byte
                vec.push(self::END_BYTE);
                // returns vec with transfer bytes
                return vec
                
                
            }

            // for Files Package
            Package::Doc(doc) =>  {
                // Converts data to bytes
                let mut vec = doc.as_bytes().to_vec();
                // Places the File Bytes
                vec.insert(0, FILE_BYTE);
                // PLaces the end byte
                vec.push(END_BYTE);
                // returns vec with transfer bytes
                return vec
            }


            // For msg Package
            Package::Msg(msg) =>  {
                // Converts data to bytes
                let mut vec = msg.as_bytes().to_vec();
                // Places Msg Byte
                vec.insert(0, STR_BYTE);
                // Places end Bytes
                vec.push(END_BYTE);
                // returns vec with transfer bytes
                return vec
                
                
            }

        }
    }


    /// Returns String of data held in Package
    ///
    /// Example
    /// ```
    /// let pack = Package::new_msg("Hello");
    /// let data = pack.to_string();
    /// assert_eq!(&data, "Hello");
    /// ```
    pub fn to_string(self) -> String {

        match self {
        
            Package::Command(temp) => return temp,
            Package::Msg(temp) => return temp,
            Package::Doc(temp) => return temp


        }
    }

}
