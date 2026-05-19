#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pgm_data: [PgmData; 8],
    pgm_check_value: [PgmCheckValue; 3],
    rd_wr_dis: RdWrDis,
    rd_repeat_data0: RdRepeatData0,
    rd_repeat_data1: RdRepeatData1,
    rd_repeat_data2: RdRepeatData2,
    rd_repeat_data3: RdRepeatData3,
    rd_repeat_data4: RdRepeatData4,
    rd_mac_sys0: RdMacSys0,
    rd_mac_sys1: RdMacSys1,
    rd_mac_sys2: RdMacSys2,
    rd_mac_sys3: RdMacSys3,
    rd_mac_sys4: RdMacSys4,
    rd_mac_sys5: RdMacSys5,
    rd_sys_part1_data: [RdSysPart1Data; 8],
    rd_usr_data: [RdUsrData; 8],
    rd_key0_data: [RdKey0Data; 8],
    rd_key1_data: [RdKey1Data; 8],
    rd_key2_data: [RdKey2Data; 8],
    rd_key3_data: [RdKey3Data; 8],
    rd_key4_data: [RdKey4Data; 8],
    rd_key5_data: [RdKey5Data; 8],
    rd_sys_part2_data0: RdSysPart2Data0,
    rd_sys_part2_data1: RdSysPart2Data1,
    rd_sys_part2_data2: RdSysPart2Data2,
    rd_sys_part2_data3: RdSysPart2Data3,
    rd_sys_part2_data4: RdSysPart2Data4,
    rd_sys_part2_data5: RdSysPart2Data5,
    rd_sys_part2_data6: RdSysPart2Data6,
    rd_sys_part2_data7: RdSysPart2Data7,
    rd_repeat_data_err0: RdRepeatDataErr0,
    rd_repeat_data_err1: RdRepeatDataErr1,
    rd_repeat_data_err2: RdRepeatDataErr2,
    rd_repeat_data_err3: RdRepeatDataErr3,
    rd_repeat_data_err4: RdRepeatDataErr4,
    _reserved35: [u8; 0x20],
    ecdsa: Ecdsa,
    _reserved36: [u8; 0x0c],
    rd_rs_data_err0: RdRsDataErr0,
    rd_rs_data_err1: RdRsDataErr1,
    clk: Clk,
    conf: Conf,
    status: Status,
    cmd: Cmd,
    int_raw: IntRaw,
    int_st: IntSt,
    int_ena: IntEna,
    int_clr: IntClr,
    dac_conf: DacConf,
    rd_tim_conf: RdTimConf,
    wr_tim_conf1: WrTimConf1,
    wr_tim_conf2: WrTimConf2,
    wr_tim_conf0_rs_bypass: WrTimConf0RsBypass,
    date: Date,
    _reserved52: [u8; 0x0600],
    apb2otp_wr_dis: Apb2otpWrDis,
    apb2otp_blk0_backup1_w1: Apb2otpBlk0Backup1W1,
    apb2otp_blk0_backup1_w2: Apb2otpBlk0Backup1W2,
    apb2otp_blk0_backup1_w3: Apb2otpBlk0Backup1W3,
    apb2otp_blk0_backup1_w4: Apb2otpBlk0Backup1W4,
    apb2otp_blk0_backup1_w5: Apb2otpBlk0Backup1W5,
    apb2otp_blk0_backup2_w1: Apb2otpBlk0Backup2W1,
    apb2otp_blk0_backup2_w2: Apb2otpBlk0Backup2W2,
    apb2otp_blk0_backup2_w3: Apb2otpBlk0Backup2W3,
    apb2otp_blk0_backup2_w4: Apb2otpBlk0Backup2W4,
    apb2otp_blk0_backup2_w5: Apb2otpBlk0Backup2W5,
    apb2otp_blk0_backup3_w1: Apb2otpBlk0Backup3W1,
    apb2otp_blk0_backup3_w2: Apb2otpBlk0Backup3W2,
    apb2otp_blk0_backup3_w3: Apb2otpBlk0Backup3W3,
    apb2otp_blk0_backup3_w4: Apb2otpBlk0Backup3W4,
    apb2otp_blk0_backup3_w5: Apb2otpBlk0Backup3W5,
    apb2otp_blk0_backup4_w1: Apb2otpBlk0Backup4W1,
    apb2otp_blk0_backup4_w2: Apb2otpBlk0Backup4W2,
    apb2otp_blk0_backup4_w3: Apb2otpBlk0Backup4W3,
    apb2otp_blk0_backup4_w4: Apb2otpBlk0Backup4W4,
    apb2otp_blk0_backup4_w5: Apb2otpBlk0Backup4W5,
    apb2otp_blk1_w1: Apb2otpBlk1W1,
    apb2otp_blk1_w2: Apb2otpBlk1W2,
    apb2otp_blk1_w3: Apb2otpBlk1W3,
    apb2otp_blk1_w4: Apb2otpBlk1W4,
    apb2otp_blk1_w5: Apb2otpBlk1W5,
    apb2otp_blk1_w6: Apb2otpBlk1W6,
    apb2otp_blk1_w7: Apb2otpBlk1W7,
    apb2otp_blk1_w8: Apb2otpBlk1W8,
    apb2otp_blk1_w9: Apb2otpBlk1W9,
    apb2otp_blk2_w1: Apb2otpBlk2W1,
    apb2otp_blk2_w2: Apb2otpBlk2W2,
    apb2otp_blk2_w3: Apb2otpBlk2W3,
    apb2otp_blk2_w4: Apb2otpBlk2W4,
    apb2otp_blk2_w5: Apb2otpBlk2W5,
    apb2otp_blk2_w6: Apb2otpBlk2W6,
    apb2otp_blk2_w7: Apb2otpBlk2W7,
    apb2otp_blk2_w8: Apb2otpBlk2W8,
    apb2otp_blk2_w9: Apb2otpBlk2W9,
    apb2otp_blk2_w10: Apb2otpBlk2W10,
    apb2otp_blk2_w11: Apb2otpBlk2W11,
    apb2otp_blk3_w1: Apb2otpBlk3W1,
    apb2otp_blk3_w2: Apb2otpBlk3W2,
    apb2otp_blk3_w3: Apb2otpBlk3W3,
    apb2otp_blk3_w4: Apb2otpBlk3W4,
    apb2otp_blk3_w5: Apb2otpBlk3W5,
    apb2otp_blk3_w6: Apb2otpBlk3W6,
    apb2otp_blk3_w7: Apb2otpBlk3W7,
    apb2otp_blk3_w8: Apb2otpBlk3W8,
    apb2otp_blk3_w9: Apb2otpBlk3W9,
    apb2otp_blk3_w10: Apb2otpBlk3W10,
    apb2otp_blk3_w11: Apb2otpBlk3W11,
    apb2otp_blk4_w1: Apb2otpBlk4W1,
    apb2otp_blk4_w2: Apb2otpBlk4W2,
    apb2otp_blk4_w3: Apb2otpBlk4W3,
    apb2otp_blk4_w4: Apb2otpBlk4W4,
    apb2otp_blk4_w5: Apb2otpBlk4W5,
    apb2otp_blk4_w6: Apb2otpBlk4W6,
    apb2otp_blk4_w7: Apb2otpBlk4W7,
    apb2otp_blk4_w8: Apb2otpBlk4W8,
    apb2otp_blk4_w9: Apb2otpBlk4W9,
    apb2otp_blk4_w10: Apb2otpBlk4W10,
    apb2otp_blk4_w11: Apb2otpBlk4W11,
    apb2otp_blk5_w1: Apb2otpBlk5W1,
    apb2otp_blk5_w2: Apb2otpBlk5W2,
    apb2otp_blk5_w3: Apb2otpBlk5W3,
    apb2otp_blk5_w4: Apb2otpBlk5W4,
    apb2otp_blk5_w5: Apb2otpBlk5W5,
    apb2otp_blk5_w6: Apb2otpBlk5W6,
    apb2otp_blk5_w7: Apb2otpBlk5W7,
    apb2otp_blk5_w8: Apb2otpBlk5W8,
    apb2otp_blk5_w9: Apb2otpBlk5W9,
    apb2otp_blk5_w10: Apb2otpBlk5W10,
    apb2otp_blk5_w11: Apb2otpBlk5W11,
    apb2otp_blk6_w1: Apb2otpBlk6W1,
    apb2otp_blk6_w2: Apb2otpBlk6W2,
    apb2otp_blk6_w3: Apb2otpBlk6W3,
    apb2otp_blk6_w4: Apb2otpBlk6W4,
    apb2otp_blk6_w5: Apb2otpBlk6W5,
    apb2otp_blk6_w6: Apb2otpBlk6W6,
    apb2otp_blk6_w7: Apb2otpBlk6W7,
    apb2otp_blk6_w8: Apb2otpBlk6W8,
    apb2otp_blk6_w9: Apb2otpBlk6W9,
    apb2otp_blk6_w10: Apb2otpBlk6W10,
    apb2otp_blk6_w11: Apb2otpBlk6W11,
    apb2otp_blk7_w1: Apb2otpBlk7W1,
    apb2otp_blk7_w2: Apb2otpBlk7W2,
    apb2otp_blk7_w3: Apb2otpBlk7W3,
    apb2otp_blk7_w4: Apb2otpBlk7W4,
    apb2otp_blk7_w5: Apb2otpBlk7W5,
    apb2otp_blk7_w6: Apb2otpBlk7W6,
    apb2otp_blk7_w7: Apb2otpBlk7W7,
    apb2otp_blk7_w8: Apb2otpBlk7W8,
    apb2otp_blk7_w9: Apb2otpBlk7W9,
    apb2otp_blk7_w10: Apb2otpBlk7W10,
    apb2otp_blk7_w11: Apb2otpBlk7W11,
    apb2otp_blk8_w1: Apb2otpBlk8W1,
    apb2otp_blk8_w2: Apb2otpBlk8W2,
    apb2otp_blk8_w3: Apb2otpBlk8W3,
    apb2otp_blk8_w4: Apb2otpBlk8W4,
    apb2otp_blk8_w5: Apb2otpBlk8W5,
    apb2otp_blk8_w6: Apb2otpBlk8W6,
    apb2otp_blk8_w7: Apb2otpBlk8W7,
    apb2otp_blk8_w8: Apb2otpBlk8W8,
    apb2otp_blk8_w9: Apb2otpBlk8W9,
    apb2otp_blk8_w10: Apb2otpBlk8W10,
    apb2otp_blk8_w11: Apb2otpBlk8W11,
    apb2otp_blk9_w1: Apb2otpBlk9W1,
    apb2otp_blk9_w2: Apb2otpBlk9W2,
    apb2otp_blk9_w3: Apb2otpBlk9W3,
    apb2otp_blk9_w4: Apb2otpBlk9W4,
    apb2otp_blk9_w5: Apb2otpBlk9W5,
    apb2otp_blk9_w6: Apb2otpBlk9W6,
    apb2otp_blk9_w7: Apb2otpBlk9W7,
    apb2otp_blk9_w8: Apb2otpBlk9W8,
    apb2otp_blk9_w9: Apb2otpBlk9W9,
    apb2otp_blk9_w10: Apb2otpBlk9W10,
    apb2otp_blk9_w11: Apb2otpBlk9W11,
    apb2otp_blk10_w1: Apb2otpBlk10W1,
    apb2otp_blk10_w2: Apb2otpBlk10W2,
    apb2otp_blk10_w3: Apb2otpBlk10W3,
    apb2otp_blk10_w4: Apb2otpBlk10W4,
    apb2otp_blk10_w5: Apb2otpBlk10W5,
    apb2otp_blk10_w6: Apb2otpBlk10W6,
    apb2otp_blk10_w7: Apb2otpBlk10W7,
    apb2otp_blk10_w8: Apb2otpBlk10W8,
    apb2otp_blk10_w9: Apb2otpBlk10W9,
    apb2otp_blk10_w10: Apb2otpBlk10W10,
    apb2otp_blk10_w11: Apb2otpBlk10W11,
    _reserved181: [u8; 0x04],
    apb2otp_en: Apb2otpEn,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - Represents pgm_data%s"]
    #[inline(always)]
    pub const fn pgm_data(&self, n: usize) -> &PgmData {
        &self.pgm_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Represents pgm_data%s"]
    #[inline(always)]
    pub fn pgm_data_iter(&self) -> impl Iterator<Item = &PgmData> {
        self.pgm_data.iter()
    }
    #[doc = "0x20..0x2c - Represents pgm_check_value%s"]
    #[inline(always)]
    pub const fn pgm_check_value(&self, n: usize) -> &PgmCheckValue {
        &self.pgm_check_value[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x2c - Represents pgm_check_value%s"]
    #[inline(always)]
    pub fn pgm_check_value_iter(&self) -> impl Iterator<Item = &PgmCheckValue> {
        self.pgm_check_value.iter()
    }
    #[doc = "0x2c - Represents rd_wr_dis"]
    #[inline(always)]
    pub const fn rd_wr_dis(&self) -> &RdWrDis {
        &self.rd_wr_dis
    }
    #[doc = "0x30 - Represents rd_repeat_data"]
    #[inline(always)]
    pub const fn rd_repeat_data0(&self) -> &RdRepeatData0 {
        &self.rd_repeat_data0
    }
    #[doc = "0x34 - Represents rd_repeat_data"]
    #[inline(always)]
    pub const fn rd_repeat_data1(&self) -> &RdRepeatData1 {
        &self.rd_repeat_data1
    }
    #[doc = "0x38 - Represents rd_repeat_data"]
    #[inline(always)]
    pub const fn rd_repeat_data2(&self) -> &RdRepeatData2 {
        &self.rd_repeat_data2
    }
    #[doc = "0x3c - Represents rd_repeat_data"]
    #[inline(always)]
    pub const fn rd_repeat_data3(&self) -> &RdRepeatData3 {
        &self.rd_repeat_data3
    }
    #[doc = "0x40 - Represents rd_repeat_data"]
    #[inline(always)]
    pub const fn rd_repeat_data4(&self) -> &RdRepeatData4 {
        &self.rd_repeat_data4
    }
    #[doc = "0x44 - Represents rd_mac_sys"]
    #[inline(always)]
    pub const fn rd_mac_sys0(&self) -> &RdMacSys0 {
        &self.rd_mac_sys0
    }
    #[doc = "0x48 - Represents rd_mac_sys"]
    #[inline(always)]
    pub const fn rd_mac_sys1(&self) -> &RdMacSys1 {
        &self.rd_mac_sys1
    }
    #[doc = "0x4c - Represents rd_mac_sys"]
    #[inline(always)]
    pub const fn rd_mac_sys2(&self) -> &RdMacSys2 {
        &self.rd_mac_sys2
    }
    #[doc = "0x50 - Represents rd_mac_sys"]
    #[inline(always)]
    pub const fn rd_mac_sys3(&self) -> &RdMacSys3 {
        &self.rd_mac_sys3
    }
    #[doc = "0x54 - Represents rd_mac_sys"]
    #[inline(always)]
    pub const fn rd_mac_sys4(&self) -> &RdMacSys4 {
        &self.rd_mac_sys4
    }
    #[doc = "0x58 - Represents rd_mac_sys"]
    #[inline(always)]
    pub const fn rd_mac_sys5(&self) -> &RdMacSys5 {
        &self.rd_mac_sys5
    }
    #[doc = "0x5c..0x7c - Represents rd_sys_part1_data%s"]
    #[inline(always)]
    pub const fn rd_sys_part1_data(&self, n: usize) -> &RdSysPart1Data {
        &self.rd_sys_part1_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5c..0x7c - Represents rd_sys_part1_data%s"]
    #[inline(always)]
    pub fn rd_sys_part1_data_iter(&self) -> impl Iterator<Item = &RdSysPart1Data> {
        self.rd_sys_part1_data.iter()
    }
    #[doc = "0x7c..0x9c - Represents rd_usr_data%s"]
    #[inline(always)]
    pub const fn rd_usr_data(&self, n: usize) -> &RdUsrData {
        &self.rd_usr_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x7c..0x9c - Represents rd_usr_data%s"]
    #[inline(always)]
    pub fn rd_usr_data_iter(&self) -> impl Iterator<Item = &RdUsrData> {
        self.rd_usr_data.iter()
    }
    #[doc = "0x9c..0xbc - Represents rd_key0_data%s"]
    #[inline(always)]
    pub const fn rd_key0_data(&self, n: usize) -> &RdKey0Data {
        &self.rd_key0_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9c..0xbc - Represents rd_key0_data%s"]
    #[inline(always)]
    pub fn rd_key0_data_iter(&self) -> impl Iterator<Item = &RdKey0Data> {
        self.rd_key0_data.iter()
    }
    #[doc = "0xbc..0xdc - Represents rd_key1_data%s"]
    #[inline(always)]
    pub const fn rd_key1_data(&self, n: usize) -> &RdKey1Data {
        &self.rd_key1_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xbc..0xdc - Represents rd_key1_data%s"]
    #[inline(always)]
    pub fn rd_key1_data_iter(&self) -> impl Iterator<Item = &RdKey1Data> {
        self.rd_key1_data.iter()
    }
    #[doc = "0xdc..0xfc - Represents rd_key2_data%s"]
    #[inline(always)]
    pub const fn rd_key2_data(&self, n: usize) -> &RdKey2Data {
        &self.rd_key2_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xdc..0xfc - Represents rd_key2_data%s"]
    #[inline(always)]
    pub fn rd_key2_data_iter(&self) -> impl Iterator<Item = &RdKey2Data> {
        self.rd_key2_data.iter()
    }
    #[doc = "0xfc..0x11c - Represents rd_key3_data%s"]
    #[inline(always)]
    pub const fn rd_key3_data(&self, n: usize) -> &RdKey3Data {
        &self.rd_key3_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xfc..0x11c - Represents rd_key3_data%s"]
    #[inline(always)]
    pub fn rd_key3_data_iter(&self) -> impl Iterator<Item = &RdKey3Data> {
        self.rd_key3_data.iter()
    }
    #[doc = "0x11c..0x13c - Represents rd_key4_data%s"]
    #[inline(always)]
    pub const fn rd_key4_data(&self, n: usize) -> &RdKey4Data {
        &self.rd_key4_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x11c..0x13c - Represents rd_key4_data%s"]
    #[inline(always)]
    pub fn rd_key4_data_iter(&self) -> impl Iterator<Item = &RdKey4Data> {
        self.rd_key4_data.iter()
    }
    #[doc = "0x13c..0x15c - Represents rd_key5_data%s"]
    #[inline(always)]
    pub const fn rd_key5_data(&self, n: usize) -> &RdKey5Data {
        &self.rd_key5_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x13c..0x15c - Represents rd_key5_data%s"]
    #[inline(always)]
    pub fn rd_key5_data_iter(&self) -> impl Iterator<Item = &RdKey5Data> {
        self.rd_key5_data.iter()
    }
    #[doc = "0x15c - Represents rd_sys_part2_data"]
    #[inline(always)]
    pub const fn rd_sys_part2_data0(&self) -> &RdSysPart2Data0 {
        &self.rd_sys_part2_data0
    }
    #[doc = "0x160 - Represents rd_sys_part2_data"]
    #[inline(always)]
    pub const fn rd_sys_part2_data1(&self) -> &RdSysPart2Data1 {
        &self.rd_sys_part2_data1
    }
    #[doc = "0x164 - Represents rd_sys_part2_data"]
    #[inline(always)]
    pub const fn rd_sys_part2_data2(&self) -> &RdSysPart2Data2 {
        &self.rd_sys_part2_data2
    }
    #[doc = "0x168 - Represents rd_sys_part2_data"]
    #[inline(always)]
    pub const fn rd_sys_part2_data3(&self) -> &RdSysPart2Data3 {
        &self.rd_sys_part2_data3
    }
    #[doc = "0x16c - Represents rd_sys_part2_data"]
    #[inline(always)]
    pub const fn rd_sys_part2_data4(&self) -> &RdSysPart2Data4 {
        &self.rd_sys_part2_data4
    }
    #[doc = "0x170 - Represents rd_sys_part2_data"]
    #[inline(always)]
    pub const fn rd_sys_part2_data5(&self) -> &RdSysPart2Data5 {
        &self.rd_sys_part2_data5
    }
    #[doc = "0x174 - Represents rd_sys_part2_data"]
    #[inline(always)]
    pub const fn rd_sys_part2_data6(&self) -> &RdSysPart2Data6 {
        &self.rd_sys_part2_data6
    }
    #[doc = "0x178 - Represents rd_sys_part2_data"]
    #[inline(always)]
    pub const fn rd_sys_part2_data7(&self) -> &RdSysPart2Data7 {
        &self.rd_sys_part2_data7
    }
    #[doc = "0x17c - Represents rd_repeat_data_err"]
    #[inline(always)]
    pub const fn rd_repeat_data_err0(&self) -> &RdRepeatDataErr0 {
        &self.rd_repeat_data_err0
    }
    #[doc = "0x180 - Represents rd_repeat_data_err"]
    #[inline(always)]
    pub const fn rd_repeat_data_err1(&self) -> &RdRepeatDataErr1 {
        &self.rd_repeat_data_err1
    }
    #[doc = "0x184 - Represents rd_repeat_data_err"]
    #[inline(always)]
    pub const fn rd_repeat_data_err2(&self) -> &RdRepeatDataErr2 {
        &self.rd_repeat_data_err2
    }
    #[doc = "0x188 - Represents rd_repeat_data_err"]
    #[inline(always)]
    pub const fn rd_repeat_data_err3(&self) -> &RdRepeatDataErr3 {
        &self.rd_repeat_data_err3
    }
    #[doc = "0x18c - Represents rd_repeat_data_err"]
    #[inline(always)]
    pub const fn rd_repeat_data_err4(&self) -> &RdRepeatDataErr4 {
        &self.rd_repeat_data_err4
    }
    #[doc = "0x1b0 - eFuse status register."]
    #[inline(always)]
    pub const fn ecdsa(&self) -> &Ecdsa {
        &self.ecdsa
    }
    #[doc = "0x1c0 - Represents rd_rs_data_err"]
    #[inline(always)]
    pub const fn rd_rs_data_err0(&self) -> &RdRsDataErr0 {
        &self.rd_rs_data_err0
    }
    #[doc = "0x1c4 - Represents rd_rs_data_err"]
    #[inline(always)]
    pub const fn rd_rs_data_err1(&self) -> &RdRsDataErr1 {
        &self.rd_rs_data_err1
    }
    #[doc = "0x1c8 - eFuse clcok configuration register."]
    #[inline(always)]
    pub const fn clk(&self) -> &Clk {
        &self.clk
    }
    #[doc = "0x1cc - eFuse operation mode configuraiton register"]
    #[inline(always)]
    pub const fn conf(&self) -> &Conf {
        &self.conf
    }
    #[doc = "0x1d0 - eFuse status register."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x1d4 - eFuse command register."]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x1d8 - eFuse raw interrupt register."]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0x1dc - eFuse interrupt status register."]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0x1e0 - eFuse interrupt enable register."]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0x1e4 - eFuse interrupt clear register."]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0x1e8 - Controls the eFuse programming voltage."]
    #[inline(always)]
    pub const fn dac_conf(&self) -> &DacConf {
        &self.dac_conf
    }
    #[doc = "0x1ec - Configures read timing parameters."]
    #[inline(always)]
    pub const fn rd_tim_conf(&self) -> &RdTimConf {
        &self.rd_tim_conf
    }
    #[doc = "0x1f0 - Configurarion register 1 of eFuse programming timing parameters."]
    #[inline(always)]
    pub const fn wr_tim_conf1(&self) -> &WrTimConf1 {
        &self.wr_tim_conf1
    }
    #[doc = "0x1f4 - Configurarion register 2 of eFuse programming timing parameters."]
    #[inline(always)]
    pub const fn wr_tim_conf2(&self) -> &WrTimConf2 {
        &self.wr_tim_conf2
    }
    #[doc = "0x1f8 - Configurarion register0 of eFuse programming time parameters and rs bypass operation."]
    #[inline(always)]
    pub const fn wr_tim_conf0_rs_bypass(&self) -> &WrTimConf0RsBypass {
        &self.wr_tim_conf0_rs_bypass
    }
    #[doc = "0x1fc - eFuse version register."]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
    #[doc = "0x800 - eFuse apb2otp block0 data register1."]
    #[inline(always)]
    pub const fn apb2otp_wr_dis(&self) -> &Apb2otpWrDis {
        &self.apb2otp_wr_dis
    }
    #[doc = "0x804 - eFuse apb2otp block0 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup1_w1(&self) -> &Apb2otpBlk0Backup1W1 {
        &self.apb2otp_blk0_backup1_w1
    }
    #[doc = "0x808 - eFuse apb2otp block0 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup1_w2(&self) -> &Apb2otpBlk0Backup1W2 {
        &self.apb2otp_blk0_backup1_w2
    }
    #[doc = "0x80c - eFuse apb2otp block0 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup1_w3(&self) -> &Apb2otpBlk0Backup1W3 {
        &self.apb2otp_blk0_backup1_w3
    }
    #[doc = "0x810 - eFuse apb2otp block0 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup1_w4(&self) -> &Apb2otpBlk0Backup1W4 {
        &self.apb2otp_blk0_backup1_w4
    }
    #[doc = "0x814 - eFuse apb2otp block0 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup1_w5(&self) -> &Apb2otpBlk0Backup1W5 {
        &self.apb2otp_blk0_backup1_w5
    }
    #[doc = "0x818 - eFuse apb2otp block0 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup2_w1(&self) -> &Apb2otpBlk0Backup2W1 {
        &self.apb2otp_blk0_backup2_w1
    }
    #[doc = "0x81c - eFuse apb2otp block0 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup2_w2(&self) -> &Apb2otpBlk0Backup2W2 {
        &self.apb2otp_blk0_backup2_w2
    }
    #[doc = "0x820 - eFuse apb2otp block0 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup2_w3(&self) -> &Apb2otpBlk0Backup2W3 {
        &self.apb2otp_blk0_backup2_w3
    }
    #[doc = "0x824 - eFuse apb2otp block0 data register10."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup2_w4(&self) -> &Apb2otpBlk0Backup2W4 {
        &self.apb2otp_blk0_backup2_w4
    }
    #[doc = "0x828 - eFuse apb2otp block0 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup2_w5(&self) -> &Apb2otpBlk0Backup2W5 {
        &self.apb2otp_blk0_backup2_w5
    }
    #[doc = "0x82c - eFuse apb2otp block0 data register12."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup3_w1(&self) -> &Apb2otpBlk0Backup3W1 {
        &self.apb2otp_blk0_backup3_w1
    }
    #[doc = "0x830 - eFuse apb2otp block0 data register13."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup3_w2(&self) -> &Apb2otpBlk0Backup3W2 {
        &self.apb2otp_blk0_backup3_w2
    }
    #[doc = "0x834 - eFuse apb2otp block0 data register14."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup3_w3(&self) -> &Apb2otpBlk0Backup3W3 {
        &self.apb2otp_blk0_backup3_w3
    }
    #[doc = "0x838 - eFuse apb2otp block0 data register15."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup3_w4(&self) -> &Apb2otpBlk0Backup3W4 {
        &self.apb2otp_blk0_backup3_w4
    }
    #[doc = "0x83c - eFuse apb2otp block0 data register16."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup3_w5(&self) -> &Apb2otpBlk0Backup3W5 {
        &self.apb2otp_blk0_backup3_w5
    }
    #[doc = "0x840 - eFuse apb2otp block0 data register17."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup4_w1(&self) -> &Apb2otpBlk0Backup4W1 {
        &self.apb2otp_blk0_backup4_w1
    }
    #[doc = "0x844 - eFuse apb2otp block0 data register18."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup4_w2(&self) -> &Apb2otpBlk0Backup4W2 {
        &self.apb2otp_blk0_backup4_w2
    }
    #[doc = "0x848 - eFuse apb2otp block0 data register19."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup4_w3(&self) -> &Apb2otpBlk0Backup4W3 {
        &self.apb2otp_blk0_backup4_w3
    }
    #[doc = "0x84c - eFuse apb2otp block0 data register20."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup4_w4(&self) -> &Apb2otpBlk0Backup4W4 {
        &self.apb2otp_blk0_backup4_w4
    }
    #[doc = "0x850 - eFuse apb2otp block0 data register21."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup4_w5(&self) -> &Apb2otpBlk0Backup4W5 {
        &self.apb2otp_blk0_backup4_w5
    }
    #[doc = "0x854 - eFuse apb2otp block1 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk1_w1(&self) -> &Apb2otpBlk1W1 {
        &self.apb2otp_blk1_w1
    }
    #[doc = "0x858 - eFuse apb2otp block1 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk1_w2(&self) -> &Apb2otpBlk1W2 {
        &self.apb2otp_blk1_w2
    }
    #[doc = "0x85c - eFuse apb2otp block1 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk1_w3(&self) -> &Apb2otpBlk1W3 {
        &self.apb2otp_blk1_w3
    }
    #[doc = "0x860 - eFuse apb2otp block1 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk1_w4(&self) -> &Apb2otpBlk1W4 {
        &self.apb2otp_blk1_w4
    }
    #[doc = "0x864 - eFuse apb2otp block1 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk1_w5(&self) -> &Apb2otpBlk1W5 {
        &self.apb2otp_blk1_w5
    }
    #[doc = "0x868 - eFuse apb2otp block1 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk1_w6(&self) -> &Apb2otpBlk1W6 {
        &self.apb2otp_blk1_w6
    }
    #[doc = "0x86c - eFuse apb2otp block1 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk1_w7(&self) -> &Apb2otpBlk1W7 {
        &self.apb2otp_blk1_w7
    }
    #[doc = "0x870 - eFuse apb2otp block1 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk1_w8(&self) -> &Apb2otpBlk1W8 {
        &self.apb2otp_blk1_w8
    }
    #[doc = "0x874 - eFuse apb2otp block1 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk1_w9(&self) -> &Apb2otpBlk1W9 {
        &self.apb2otp_blk1_w9
    }
    #[doc = "0x878 - eFuse apb2otp block2 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w1(&self) -> &Apb2otpBlk2W1 {
        &self.apb2otp_blk2_w1
    }
    #[doc = "0x87c - eFuse apb2otp block2 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w2(&self) -> &Apb2otpBlk2W2 {
        &self.apb2otp_blk2_w2
    }
    #[doc = "0x880 - eFuse apb2otp block2 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w3(&self) -> &Apb2otpBlk2W3 {
        &self.apb2otp_blk2_w3
    }
    #[doc = "0x884 - eFuse apb2otp block2 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w4(&self) -> &Apb2otpBlk2W4 {
        &self.apb2otp_blk2_w4
    }
    #[doc = "0x888 - eFuse apb2otp block2 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w5(&self) -> &Apb2otpBlk2W5 {
        &self.apb2otp_blk2_w5
    }
    #[doc = "0x88c - eFuse apb2otp block2 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w6(&self) -> &Apb2otpBlk2W6 {
        &self.apb2otp_blk2_w6
    }
    #[doc = "0x890 - eFuse apb2otp block2 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w7(&self) -> &Apb2otpBlk2W7 {
        &self.apb2otp_blk2_w7
    }
    #[doc = "0x894 - eFuse apb2otp block2 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w8(&self) -> &Apb2otpBlk2W8 {
        &self.apb2otp_blk2_w8
    }
    #[doc = "0x898 - eFuse apb2otp block2 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w9(&self) -> &Apb2otpBlk2W9 {
        &self.apb2otp_blk2_w9
    }
    #[doc = "0x89c - eFuse apb2otp block2 data register10."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w10(&self) -> &Apb2otpBlk2W10 {
        &self.apb2otp_blk2_w10
    }
    #[doc = "0x8a0 - eFuse apb2otp block2 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w11(&self) -> &Apb2otpBlk2W11 {
        &self.apb2otp_blk2_w11
    }
    #[doc = "0x8a4 - eFuse apb2otp block3 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w1(&self) -> &Apb2otpBlk3W1 {
        &self.apb2otp_blk3_w1
    }
    #[doc = "0x8a8 - eFuse apb2otp block3 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w2(&self) -> &Apb2otpBlk3W2 {
        &self.apb2otp_blk3_w2
    }
    #[doc = "0x8ac - eFuse apb2otp block3 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w3(&self) -> &Apb2otpBlk3W3 {
        &self.apb2otp_blk3_w3
    }
    #[doc = "0x8b0 - eFuse apb2otp block3 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w4(&self) -> &Apb2otpBlk3W4 {
        &self.apb2otp_blk3_w4
    }
    #[doc = "0x8b4 - eFuse apb2otp block3 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w5(&self) -> &Apb2otpBlk3W5 {
        &self.apb2otp_blk3_w5
    }
    #[doc = "0x8b8 - eFuse apb2otp block3 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w6(&self) -> &Apb2otpBlk3W6 {
        &self.apb2otp_blk3_w6
    }
    #[doc = "0x8bc - eFuse apb2otp block3 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w7(&self) -> &Apb2otpBlk3W7 {
        &self.apb2otp_blk3_w7
    }
    #[doc = "0x8c0 - eFuse apb2otp block3 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w8(&self) -> &Apb2otpBlk3W8 {
        &self.apb2otp_blk3_w8
    }
    #[doc = "0x8c4 - eFuse apb2otp block3 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w9(&self) -> &Apb2otpBlk3W9 {
        &self.apb2otp_blk3_w9
    }
    #[doc = "0x8c8 - eFuse apb2otp block3 data register10."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w10(&self) -> &Apb2otpBlk3W10 {
        &self.apb2otp_blk3_w10
    }
    #[doc = "0x8cc - eFuse apb2otp block3 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w11(&self) -> &Apb2otpBlk3W11 {
        &self.apb2otp_blk3_w11
    }
    #[doc = "0x8d0 - eFuse apb2otp block4 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w1(&self) -> &Apb2otpBlk4W1 {
        &self.apb2otp_blk4_w1
    }
    #[doc = "0x8d4 - eFuse apb2otp block4 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w2(&self) -> &Apb2otpBlk4W2 {
        &self.apb2otp_blk4_w2
    }
    #[doc = "0x8d8 - eFuse apb2otp block4 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w3(&self) -> &Apb2otpBlk4W3 {
        &self.apb2otp_blk4_w3
    }
    #[doc = "0x8dc - eFuse apb2otp block4 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w4(&self) -> &Apb2otpBlk4W4 {
        &self.apb2otp_blk4_w4
    }
    #[doc = "0x8e0 - eFuse apb2otp block4 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w5(&self) -> &Apb2otpBlk4W5 {
        &self.apb2otp_blk4_w5
    }
    #[doc = "0x8e4 - eFuse apb2otp block4 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w6(&self) -> &Apb2otpBlk4W6 {
        &self.apb2otp_blk4_w6
    }
    #[doc = "0x8e8 - eFuse apb2otp block4 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w7(&self) -> &Apb2otpBlk4W7 {
        &self.apb2otp_blk4_w7
    }
    #[doc = "0x8ec - eFuse apb2otp block4 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w8(&self) -> &Apb2otpBlk4W8 {
        &self.apb2otp_blk4_w8
    }
    #[doc = "0x8f0 - eFuse apb2otp block4 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w9(&self) -> &Apb2otpBlk4W9 {
        &self.apb2otp_blk4_w9
    }
    #[doc = "0x8f4 - eFuse apb2otp block4 data registe10."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w10(&self) -> &Apb2otpBlk4W10 {
        &self.apb2otp_blk4_w10
    }
    #[doc = "0x8f8 - eFuse apb2otp block4 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w11(&self) -> &Apb2otpBlk4W11 {
        &self.apb2otp_blk4_w11
    }
    #[doc = "0x8fc - eFuse apb2otp block5 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w1(&self) -> &Apb2otpBlk5W1 {
        &self.apb2otp_blk5_w1
    }
    #[doc = "0x900 - eFuse apb2otp block5 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w2(&self) -> &Apb2otpBlk5W2 {
        &self.apb2otp_blk5_w2
    }
    #[doc = "0x904 - eFuse apb2otp block5 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w3(&self) -> &Apb2otpBlk5W3 {
        &self.apb2otp_blk5_w3
    }
    #[doc = "0x908 - eFuse apb2otp block5 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w4(&self) -> &Apb2otpBlk5W4 {
        &self.apb2otp_blk5_w4
    }
    #[doc = "0x90c - eFuse apb2otp block5 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w5(&self) -> &Apb2otpBlk5W5 {
        &self.apb2otp_blk5_w5
    }
    #[doc = "0x910 - eFuse apb2otp block5 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w6(&self) -> &Apb2otpBlk5W6 {
        &self.apb2otp_blk5_w6
    }
    #[doc = "0x914 - eFuse apb2otp block5 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w7(&self) -> &Apb2otpBlk5W7 {
        &self.apb2otp_blk5_w7
    }
    #[doc = "0x918 - eFuse apb2otp block5 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w8(&self) -> &Apb2otpBlk5W8 {
        &self.apb2otp_blk5_w8
    }
    #[doc = "0x91c - eFuse apb2otp block5 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w9(&self) -> &Apb2otpBlk5W9 {
        &self.apb2otp_blk5_w9
    }
    #[doc = "0x920 - eFuse apb2otp block5 data register10."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w10(&self) -> &Apb2otpBlk5W10 {
        &self.apb2otp_blk5_w10
    }
    #[doc = "0x924 - eFuse apb2otp block5 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w11(&self) -> &Apb2otpBlk5W11 {
        &self.apb2otp_blk5_w11
    }
    #[doc = "0x928 - eFuse apb2otp block6 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w1(&self) -> &Apb2otpBlk6W1 {
        &self.apb2otp_blk6_w1
    }
    #[doc = "0x92c - eFuse apb2otp block6 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w2(&self) -> &Apb2otpBlk6W2 {
        &self.apb2otp_blk6_w2
    }
    #[doc = "0x930 - eFuse apb2otp block6 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w3(&self) -> &Apb2otpBlk6W3 {
        &self.apb2otp_blk6_w3
    }
    #[doc = "0x934 - eFuse apb2otp block6 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w4(&self) -> &Apb2otpBlk6W4 {
        &self.apb2otp_blk6_w4
    }
    #[doc = "0x938 - eFuse apb2otp block6 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w5(&self) -> &Apb2otpBlk6W5 {
        &self.apb2otp_blk6_w5
    }
    #[doc = "0x93c - eFuse apb2otp block6 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w6(&self) -> &Apb2otpBlk6W6 {
        &self.apb2otp_blk6_w6
    }
    #[doc = "0x940 - eFuse apb2otp block6 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w7(&self) -> &Apb2otpBlk6W7 {
        &self.apb2otp_blk6_w7
    }
    #[doc = "0x944 - eFuse apb2otp block6 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w8(&self) -> &Apb2otpBlk6W8 {
        &self.apb2otp_blk6_w8
    }
    #[doc = "0x948 - eFuse apb2otp block6 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w9(&self) -> &Apb2otpBlk6W9 {
        &self.apb2otp_blk6_w9
    }
    #[doc = "0x94c - eFuse apb2otp block6 data register10."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w10(&self) -> &Apb2otpBlk6W10 {
        &self.apb2otp_blk6_w10
    }
    #[doc = "0x950 - eFuse apb2otp block6 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w11(&self) -> &Apb2otpBlk6W11 {
        &self.apb2otp_blk6_w11
    }
    #[doc = "0x954 - eFuse apb2otp block7 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w1(&self) -> &Apb2otpBlk7W1 {
        &self.apb2otp_blk7_w1
    }
    #[doc = "0x958 - eFuse apb2otp block7 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w2(&self) -> &Apb2otpBlk7W2 {
        &self.apb2otp_blk7_w2
    }
    #[doc = "0x95c - eFuse apb2otp block7 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w3(&self) -> &Apb2otpBlk7W3 {
        &self.apb2otp_blk7_w3
    }
    #[doc = "0x960 - eFuse apb2otp block7 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w4(&self) -> &Apb2otpBlk7W4 {
        &self.apb2otp_blk7_w4
    }
    #[doc = "0x964 - eFuse apb2otp block7 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w5(&self) -> &Apb2otpBlk7W5 {
        &self.apb2otp_blk7_w5
    }
    #[doc = "0x968 - eFuse apb2otp block7 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w6(&self) -> &Apb2otpBlk7W6 {
        &self.apb2otp_blk7_w6
    }
    #[doc = "0x96c - eFuse apb2otp block7 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w7(&self) -> &Apb2otpBlk7W7 {
        &self.apb2otp_blk7_w7
    }
    #[doc = "0x970 - eFuse apb2otp block7 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w8(&self) -> &Apb2otpBlk7W8 {
        &self.apb2otp_blk7_w8
    }
    #[doc = "0x974 - eFuse apb2otp block7 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w9(&self) -> &Apb2otpBlk7W9 {
        &self.apb2otp_blk7_w9
    }
    #[doc = "0x978 - eFuse apb2otp block7 data register10."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w10(&self) -> &Apb2otpBlk7W10 {
        &self.apb2otp_blk7_w10
    }
    #[doc = "0x97c - eFuse apb2otp block7 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w11(&self) -> &Apb2otpBlk7W11 {
        &self.apb2otp_blk7_w11
    }
    #[doc = "0x980 - eFuse apb2otp block8 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w1(&self) -> &Apb2otpBlk8W1 {
        &self.apb2otp_blk8_w1
    }
    #[doc = "0x984 - eFuse apb2otp block8 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w2(&self) -> &Apb2otpBlk8W2 {
        &self.apb2otp_blk8_w2
    }
    #[doc = "0x988 - eFuse apb2otp block8 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w3(&self) -> &Apb2otpBlk8W3 {
        &self.apb2otp_blk8_w3
    }
    #[doc = "0x98c - eFuse apb2otp block8 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w4(&self) -> &Apb2otpBlk8W4 {
        &self.apb2otp_blk8_w4
    }
    #[doc = "0x990 - eFuse apb2otp block8 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w5(&self) -> &Apb2otpBlk8W5 {
        &self.apb2otp_blk8_w5
    }
    #[doc = "0x994 - eFuse apb2otp block8 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w6(&self) -> &Apb2otpBlk8W6 {
        &self.apb2otp_blk8_w6
    }
    #[doc = "0x998 - eFuse apb2otp block8 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w7(&self) -> &Apb2otpBlk8W7 {
        &self.apb2otp_blk8_w7
    }
    #[doc = "0x99c - eFuse apb2otp block8 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w8(&self) -> &Apb2otpBlk8W8 {
        &self.apb2otp_blk8_w8
    }
    #[doc = "0x9a0 - eFuse apb2otp block8 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w9(&self) -> &Apb2otpBlk8W9 {
        &self.apb2otp_blk8_w9
    }
    #[doc = "0x9a4 - eFuse apb2otp block8 data register10."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w10(&self) -> &Apb2otpBlk8W10 {
        &self.apb2otp_blk8_w10
    }
    #[doc = "0x9a8 - eFuse apb2otp block8 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w11(&self) -> &Apb2otpBlk8W11 {
        &self.apb2otp_blk8_w11
    }
    #[doc = "0x9ac - eFuse apb2otp block9 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w1(&self) -> &Apb2otpBlk9W1 {
        &self.apb2otp_blk9_w1
    }
    #[doc = "0x9b0 - eFuse apb2otp block9 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w2(&self) -> &Apb2otpBlk9W2 {
        &self.apb2otp_blk9_w2
    }
    #[doc = "0x9b4 - eFuse apb2otp block9 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w3(&self) -> &Apb2otpBlk9W3 {
        &self.apb2otp_blk9_w3
    }
    #[doc = "0x9b8 - eFuse apb2otp block9 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w4(&self) -> &Apb2otpBlk9W4 {
        &self.apb2otp_blk9_w4
    }
    #[doc = "0x9bc - eFuse apb2otp block9 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w5(&self) -> &Apb2otpBlk9W5 {
        &self.apb2otp_blk9_w5
    }
    #[doc = "0x9c0 - eFuse apb2otp block9 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w6(&self) -> &Apb2otpBlk9W6 {
        &self.apb2otp_blk9_w6
    }
    #[doc = "0x9c4 - eFuse apb2otp block9 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w7(&self) -> &Apb2otpBlk9W7 {
        &self.apb2otp_blk9_w7
    }
    #[doc = "0x9c8 - eFuse apb2otp block9 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w8(&self) -> &Apb2otpBlk9W8 {
        &self.apb2otp_blk9_w8
    }
    #[doc = "0x9cc - eFuse apb2otp block9 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w9(&self) -> &Apb2otpBlk9W9 {
        &self.apb2otp_blk9_w9
    }
    #[doc = "0x9d0 - eFuse apb2otp block9 data register10."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w10(&self) -> &Apb2otpBlk9W10 {
        &self.apb2otp_blk9_w10
    }
    #[doc = "0x9d4 - eFuse apb2otp block9 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w11(&self) -> &Apb2otpBlk9W11 {
        &self.apb2otp_blk9_w11
    }
    #[doc = "0x9d8 - eFuse apb2otp block10 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w1(&self) -> &Apb2otpBlk10W1 {
        &self.apb2otp_blk10_w1
    }
    #[doc = "0x9dc - eFuse apb2otp block10 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w2(&self) -> &Apb2otpBlk10W2 {
        &self.apb2otp_blk10_w2
    }
    #[doc = "0x9e0 - eFuse apb2otp block10 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w3(&self) -> &Apb2otpBlk10W3 {
        &self.apb2otp_blk10_w3
    }
    #[doc = "0x9e4 - eFuse apb2otp block10 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w4(&self) -> &Apb2otpBlk10W4 {
        &self.apb2otp_blk10_w4
    }
    #[doc = "0x9e8 - eFuse apb2otp block10 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w5(&self) -> &Apb2otpBlk10W5 {
        &self.apb2otp_blk10_w5
    }
    #[doc = "0x9ec - eFuse apb2otp block10 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w6(&self) -> &Apb2otpBlk10W6 {
        &self.apb2otp_blk10_w6
    }
    #[doc = "0x9f0 - eFuse apb2otp block10 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w7(&self) -> &Apb2otpBlk10W7 {
        &self.apb2otp_blk10_w7
    }
    #[doc = "0x9f4 - eFuse apb2otp block10 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w8(&self) -> &Apb2otpBlk10W8 {
        &self.apb2otp_blk10_w8
    }
    #[doc = "0x9f8 - eFuse apb2otp block10 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w9(&self) -> &Apb2otpBlk10W9 {
        &self.apb2otp_blk10_w9
    }
    #[doc = "0x9fc - eFuse apb2otp block10 data register10."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w10(&self) -> &Apb2otpBlk10W10 {
        &self.apb2otp_blk10_w10
    }
    #[doc = "0xa00 - eFuse apb2otp block10 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w11(&self) -> &Apb2otpBlk10W11 {
        &self.apb2otp_blk10_w11
    }
    #[doc = "0xa08 - eFuse apb2otp enable configuration register."]
    #[inline(always)]
    pub const fn apb2otp_en(&self) -> &Apb2otpEn {
        &self.apb2otp_en
    }
}
#[doc = "PGM_DATA (rw) register accessor: Represents pgm_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`pgm_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgm_data`] module"]
#[doc(alias = "PGM_DATA")]
pub type PgmData = crate::Reg<pgm_data::PgmDataSpec>;
#[doc = "Represents pgm_data%s"]
pub mod pgm_data;
#[doc = "PGM_CHECK_VALUE (rw) register accessor: Represents pgm_check_value%s\n\nYou can [`read`](crate::Reg::read) this register and get [`pgm_check_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_check_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgm_check_value`] module"]
#[doc(alias = "PGM_CHECK_VALUE")]
pub type PgmCheckValue = crate::Reg<pgm_check_value::PgmCheckValueSpec>;
#[doc = "Represents pgm_check_value%s"]
pub mod pgm_check_value;
#[doc = "RD_WR_DIS (r) register accessor: Represents rd_wr_dis\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_wr_dis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_wr_dis`] module"]
#[doc(alias = "RD_WR_DIS")]
pub type RdWrDis = crate::Reg<rd_wr_dis::RdWrDisSpec>;
#[doc = "Represents rd_wr_dis"]
pub mod rd_wr_dis;
#[doc = "RD_REPEAT_DATA0 (r) register accessor: Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data0`] module"]
#[doc(alias = "RD_REPEAT_DATA0")]
pub type RdRepeatData0 = crate::Reg<rd_repeat_data0::RdRepeatData0Spec>;
#[doc = "Represents rd_repeat_data"]
pub mod rd_repeat_data0;
#[doc = "RD_REPEAT_DATA1 (r) register accessor: Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data1`] module"]
#[doc(alias = "RD_REPEAT_DATA1")]
pub type RdRepeatData1 = crate::Reg<rd_repeat_data1::RdRepeatData1Spec>;
#[doc = "Represents rd_repeat_data"]
pub mod rd_repeat_data1;
#[doc = "RD_REPEAT_DATA2 (r) register accessor: Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data2`] module"]
#[doc(alias = "RD_REPEAT_DATA2")]
pub type RdRepeatData2 = crate::Reg<rd_repeat_data2::RdRepeatData2Spec>;
#[doc = "Represents rd_repeat_data"]
pub mod rd_repeat_data2;
#[doc = "RD_REPEAT_DATA3 (r) register accessor: Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data3`] module"]
#[doc(alias = "RD_REPEAT_DATA3")]
pub type RdRepeatData3 = crate::Reg<rd_repeat_data3::RdRepeatData3Spec>;
#[doc = "Represents rd_repeat_data"]
pub mod rd_repeat_data3;
#[doc = "RD_REPEAT_DATA4 (r) register accessor: Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data4`] module"]
#[doc(alias = "RD_REPEAT_DATA4")]
pub type RdRepeatData4 = crate::Reg<rd_repeat_data4::RdRepeatData4Spec>;
#[doc = "Represents rd_repeat_data"]
pub mod rd_repeat_data4;
#[doc = "RD_MAC_SYS0 (r) register accessor: Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mac_sys0`] module"]
#[doc(alias = "RD_MAC_SYS0")]
pub type RdMacSys0 = crate::Reg<rd_mac_sys0::RdMacSys0Spec>;
#[doc = "Represents rd_mac_sys"]
pub mod rd_mac_sys0;
#[doc = "RD_MAC_SYS1 (r) register accessor: Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mac_sys1`] module"]
#[doc(alias = "RD_MAC_SYS1")]
pub type RdMacSys1 = crate::Reg<rd_mac_sys1::RdMacSys1Spec>;
#[doc = "Represents rd_mac_sys"]
pub mod rd_mac_sys1;
#[doc = "RD_MAC_SYS2 (r) register accessor: Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mac_sys2`] module"]
#[doc(alias = "RD_MAC_SYS2")]
pub type RdMacSys2 = crate::Reg<rd_mac_sys2::RdMacSys2Spec>;
#[doc = "Represents rd_mac_sys"]
pub mod rd_mac_sys2;
#[doc = "RD_MAC_SYS3 (r) register accessor: Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mac_sys3`] module"]
#[doc(alias = "RD_MAC_SYS3")]
pub type RdMacSys3 = crate::Reg<rd_mac_sys3::RdMacSys3Spec>;
#[doc = "Represents rd_mac_sys"]
pub mod rd_mac_sys3;
#[doc = "RD_MAC_SYS4 (r) register accessor: Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mac_sys4`] module"]
#[doc(alias = "RD_MAC_SYS4")]
pub type RdMacSys4 = crate::Reg<rd_mac_sys4::RdMacSys4Spec>;
#[doc = "Represents rd_mac_sys"]
pub mod rd_mac_sys4;
#[doc = "RD_MAC_SYS5 (r) register accessor: Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mac_sys5`] module"]
#[doc(alias = "RD_MAC_SYS5")]
pub type RdMacSys5 = crate::Reg<rd_mac_sys5::RdMacSys5Spec>;
#[doc = "Represents rd_mac_sys"]
pub mod rd_mac_sys5;
#[doc = "RD_SYS_PART1_DATA (r) register accessor: Represents rd_sys_part1_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part1_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_sys_part1_data`] module"]
#[doc(alias = "RD_SYS_PART1_DATA")]
pub type RdSysPart1Data = crate::Reg<rd_sys_part1_data::RdSysPart1DataSpec>;
#[doc = "Represents rd_sys_part1_data%s"]
pub mod rd_sys_part1_data;
#[doc = "RD_USR_DATA (r) register accessor: Represents rd_usr_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_usr_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_usr_data`] module"]
#[doc(alias = "RD_USR_DATA")]
pub type RdUsrData = crate::Reg<rd_usr_data::RdUsrDataSpec>;
#[doc = "Represents rd_usr_data%s"]
pub mod rd_usr_data;
#[doc = "RD_KEY0_DATA (r) register accessor: Represents rd_key0_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key0_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_key0_data`] module"]
#[doc(alias = "RD_KEY0_DATA")]
pub type RdKey0Data = crate::Reg<rd_key0_data::RdKey0DataSpec>;
#[doc = "Represents rd_key0_data%s"]
pub mod rd_key0_data;
#[doc = "RD_KEY1_DATA (r) register accessor: Represents rd_key1_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key1_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_key1_data`] module"]
#[doc(alias = "RD_KEY1_DATA")]
pub type RdKey1Data = crate::Reg<rd_key1_data::RdKey1DataSpec>;
#[doc = "Represents rd_key1_data%s"]
pub mod rd_key1_data;
#[doc = "RD_KEY2_DATA (r) register accessor: Represents rd_key2_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key2_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_key2_data`] module"]
#[doc(alias = "RD_KEY2_DATA")]
pub type RdKey2Data = crate::Reg<rd_key2_data::RdKey2DataSpec>;
#[doc = "Represents rd_key2_data%s"]
pub mod rd_key2_data;
#[doc = "RD_KEY3_DATA (r) register accessor: Represents rd_key3_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key3_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_key3_data`] module"]
#[doc(alias = "RD_KEY3_DATA")]
pub type RdKey3Data = crate::Reg<rd_key3_data::RdKey3DataSpec>;
#[doc = "Represents rd_key3_data%s"]
pub mod rd_key3_data;
#[doc = "RD_KEY4_DATA (r) register accessor: Represents rd_key4_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key4_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_key4_data`] module"]
#[doc(alias = "RD_KEY4_DATA")]
pub type RdKey4Data = crate::Reg<rd_key4_data::RdKey4DataSpec>;
#[doc = "Represents rd_key4_data%s"]
pub mod rd_key4_data;
#[doc = "RD_KEY5_DATA (r) register accessor: Represents rd_key5_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key5_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_key5_data`] module"]
#[doc(alias = "RD_KEY5_DATA")]
pub type RdKey5Data = crate::Reg<rd_key5_data::RdKey5DataSpec>;
#[doc = "Represents rd_key5_data%s"]
pub mod rd_key5_data;
#[doc = "RD_SYS_PART2_DATA0 (r) register accessor: Represents rd_sys_part2_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part2_data0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_sys_part2_data0`] module"]
#[doc(alias = "RD_SYS_PART2_DATA0")]
pub type RdSysPart2Data0 = crate::Reg<rd_sys_part2_data0::RdSysPart2Data0Spec>;
#[doc = "Represents rd_sys_part2_data"]
pub mod rd_sys_part2_data0;
#[doc = "RD_SYS_PART2_DATA1 (r) register accessor: Represents rd_sys_part2_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part2_data1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_sys_part2_data1`] module"]
#[doc(alias = "RD_SYS_PART2_DATA1")]
pub type RdSysPart2Data1 = crate::Reg<rd_sys_part2_data1::RdSysPart2Data1Spec>;
#[doc = "Represents rd_sys_part2_data"]
pub mod rd_sys_part2_data1;
#[doc = "RD_SYS_PART2_DATA2 (r) register accessor: Represents rd_sys_part2_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part2_data2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_sys_part2_data2`] module"]
#[doc(alias = "RD_SYS_PART2_DATA2")]
pub type RdSysPart2Data2 = crate::Reg<rd_sys_part2_data2::RdSysPart2Data2Spec>;
#[doc = "Represents rd_sys_part2_data"]
pub mod rd_sys_part2_data2;
#[doc = "RD_SYS_PART2_DATA3 (r) register accessor: Represents rd_sys_part2_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part2_data3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_sys_part2_data3`] module"]
#[doc(alias = "RD_SYS_PART2_DATA3")]
pub type RdSysPart2Data3 = crate::Reg<rd_sys_part2_data3::RdSysPart2Data3Spec>;
#[doc = "Represents rd_sys_part2_data"]
pub mod rd_sys_part2_data3;
#[doc = "RD_SYS_PART2_DATA4 (r) register accessor: Represents rd_sys_part2_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part2_data4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_sys_part2_data4`] module"]
#[doc(alias = "RD_SYS_PART2_DATA4")]
pub type RdSysPart2Data4 = crate::Reg<rd_sys_part2_data4::RdSysPart2Data4Spec>;
#[doc = "Represents rd_sys_part2_data"]
pub mod rd_sys_part2_data4;
#[doc = "RD_SYS_PART2_DATA5 (r) register accessor: Represents rd_sys_part2_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part2_data5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_sys_part2_data5`] module"]
#[doc(alias = "RD_SYS_PART2_DATA5")]
pub type RdSysPart2Data5 = crate::Reg<rd_sys_part2_data5::RdSysPart2Data5Spec>;
#[doc = "Represents rd_sys_part2_data"]
pub mod rd_sys_part2_data5;
#[doc = "RD_SYS_PART2_DATA6 (r) register accessor: Represents rd_sys_part2_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part2_data6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_sys_part2_data6`] module"]
#[doc(alias = "RD_SYS_PART2_DATA6")]
pub type RdSysPart2Data6 = crate::Reg<rd_sys_part2_data6::RdSysPart2Data6Spec>;
#[doc = "Represents rd_sys_part2_data"]
pub mod rd_sys_part2_data6;
#[doc = "RD_SYS_PART2_DATA7 (r) register accessor: Represents rd_sys_part2_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part2_data7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_sys_part2_data7`] module"]
#[doc(alias = "RD_SYS_PART2_DATA7")]
pub type RdSysPart2Data7 = crate::Reg<rd_sys_part2_data7::RdSysPart2Data7Spec>;
#[doc = "Represents rd_sys_part2_data"]
pub mod rd_sys_part2_data7;
#[doc = "RD_REPEAT_DATA_ERR0 (r) register accessor: Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data_err0`] module"]
#[doc(alias = "RD_REPEAT_DATA_ERR0")]
pub type RdRepeatDataErr0 = crate::Reg<rd_repeat_data_err0::RdRepeatDataErr0Spec>;
#[doc = "Represents rd_repeat_data_err"]
pub mod rd_repeat_data_err0;
#[doc = "RD_REPEAT_DATA_ERR1 (r) register accessor: Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data_err1`] module"]
#[doc(alias = "RD_REPEAT_DATA_ERR1")]
pub type RdRepeatDataErr1 = crate::Reg<rd_repeat_data_err1::RdRepeatDataErr1Spec>;
#[doc = "Represents rd_repeat_data_err"]
pub mod rd_repeat_data_err1;
#[doc = "RD_REPEAT_DATA_ERR2 (r) register accessor: Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data_err2`] module"]
#[doc(alias = "RD_REPEAT_DATA_ERR2")]
pub type RdRepeatDataErr2 = crate::Reg<rd_repeat_data_err2::RdRepeatDataErr2Spec>;
#[doc = "Represents rd_repeat_data_err"]
pub mod rd_repeat_data_err2;
#[doc = "RD_REPEAT_DATA_ERR3 (r) register accessor: Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data_err3`] module"]
#[doc(alias = "RD_REPEAT_DATA_ERR3")]
pub type RdRepeatDataErr3 = crate::Reg<rd_repeat_data_err3::RdRepeatDataErr3Spec>;
#[doc = "Represents rd_repeat_data_err"]
pub mod rd_repeat_data_err3;
#[doc = "RD_REPEAT_DATA_ERR4 (r) register accessor: Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data_err4`] module"]
#[doc(alias = "RD_REPEAT_DATA_ERR4")]
pub type RdRepeatDataErr4 = crate::Reg<rd_repeat_data_err4::RdRepeatDataErr4Spec>;
#[doc = "Represents rd_repeat_data_err"]
pub mod rd_repeat_data_err4;
#[doc = "ECDSA (rw) register accessor: eFuse status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ecdsa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecdsa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecdsa`] module"]
#[doc(alias = "ECDSA")]
pub type Ecdsa = crate::Reg<ecdsa::EcdsaSpec>;
#[doc = "eFuse status register."]
pub mod ecdsa;
#[doc = "RD_RS_DATA_ERR0 (r) register accessor: Represents rd_rs_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_rs_data_err0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_rs_data_err0`] module"]
#[doc(alias = "RD_RS_DATA_ERR0")]
pub type RdRsDataErr0 = crate::Reg<rd_rs_data_err0::RdRsDataErr0Spec>;
#[doc = "Represents rd_rs_data_err"]
pub mod rd_rs_data_err0;
#[doc = "RD_RS_DATA_ERR1 (r) register accessor: Represents rd_rs_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_rs_data_err1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_rs_data_err1`] module"]
#[doc(alias = "RD_RS_DATA_ERR1")]
pub type RdRsDataErr1 = crate::Reg<rd_rs_data_err1::RdRsDataErr1Spec>;
#[doc = "Represents rd_rs_data_err"]
pub mod rd_rs_data_err1;
#[doc = "CLK (rw) register accessor: eFuse clcok configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`] module"]
#[doc(alias = "CLK")]
pub type Clk = crate::Reg<clk::ClkSpec>;
#[doc = "eFuse clcok configuration register."]
pub mod clk;
#[doc = "CONF (rw) register accessor: eFuse operation mode configuraiton register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
#[doc(alias = "CONF")]
pub type Conf = crate::Reg<conf::ConfSpec>;
#[doc = "eFuse operation mode configuraiton register"]
pub mod conf;
#[doc = "STATUS (r) register accessor: eFuse status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "eFuse status register."]
pub mod status;
#[doc = "CMD (rw) register accessor: eFuse command register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "eFuse command register."]
pub mod cmd;
#[doc = "INT_RAW (r) register accessor: eFuse raw interrupt register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
#[doc(alias = "INT_RAW")]
pub type IntRaw = crate::Reg<int_raw::IntRawSpec>;
#[doc = "eFuse raw interrupt register."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: eFuse interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::IntStSpec>;
#[doc = "eFuse interrupt status register."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: eFuse interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
#[doc(alias = "INT_ENA")]
pub type IntEna = crate::Reg<int_ena::IntEnaSpec>;
#[doc = "eFuse interrupt enable register."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: eFuse interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
#[doc = "eFuse interrupt clear register."]
pub mod int_clr;
#[doc = "DAC_CONF (rw) register accessor: Controls the eFuse programming voltage.\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_conf`] module"]
#[doc(alias = "DAC_CONF")]
pub type DacConf = crate::Reg<dac_conf::DacConfSpec>;
#[doc = "Controls the eFuse programming voltage."]
pub mod dac_conf;
#[doc = "RD_TIM_CONF (rw) register accessor: Configures read timing parameters.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_tim_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_tim_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_tim_conf`] module"]
#[doc(alias = "RD_TIM_CONF")]
pub type RdTimConf = crate::Reg<rd_tim_conf::RdTimConfSpec>;
#[doc = "Configures read timing parameters."]
pub mod rd_tim_conf;
#[doc = "WR_TIM_CONF1 (rw) register accessor: Configurarion register 1 of eFuse programming timing parameters.\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_tim_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_tim_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_tim_conf1`] module"]
#[doc(alias = "WR_TIM_CONF1")]
pub type WrTimConf1 = crate::Reg<wr_tim_conf1::WrTimConf1Spec>;
#[doc = "Configurarion register 1 of eFuse programming timing parameters."]
pub mod wr_tim_conf1;
#[doc = "WR_TIM_CONF2 (rw) register accessor: Configurarion register 2 of eFuse programming timing parameters.\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_tim_conf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_tim_conf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_tim_conf2`] module"]
#[doc(alias = "WR_TIM_CONF2")]
pub type WrTimConf2 = crate::Reg<wr_tim_conf2::WrTimConf2Spec>;
#[doc = "Configurarion register 2 of eFuse programming timing parameters."]
pub mod wr_tim_conf2;
#[doc = "WR_TIM_CONF0_RS_BYPASS (rw) register accessor: Configurarion register0 of eFuse programming time parameters and rs bypass operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_tim_conf0_rs_bypass::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_tim_conf0_rs_bypass::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_tim_conf0_rs_bypass`] module"]
#[doc(alias = "WR_TIM_CONF0_RS_BYPASS")]
pub type WrTimConf0RsBypass = crate::Reg<wr_tim_conf0_rs_bypass::WrTimConf0RsBypassSpec>;
#[doc = "Configurarion register0 of eFuse programming time parameters and rs bypass operation."]
pub mod wr_tim_conf0_rs_bypass;
#[doc = "DATE (rw) register accessor: eFuse version register.\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "eFuse version register."]
pub mod date;
#[doc = "APB2OTP_WR_DIS (r) register accessor: eFuse apb2otp block0 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_wr_dis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_wr_dis`] module"]
#[doc(alias = "APB2OTP_WR_DIS")]
pub type Apb2otpWrDis = crate::Reg<apb2otp_wr_dis::Apb2otpWrDisSpec>;
#[doc = "eFuse apb2otp block0 data register1."]
pub mod apb2otp_wr_dis;
#[doc = "APB2OTP_BLK0_BACKUP1_W1 (r) register accessor: eFuse apb2otp block0 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup1_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup1_w1`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP1_W1")]
pub type Apb2otpBlk0Backup1W1 = crate::Reg<apb2otp_blk0_backup1_w1::Apb2otpBlk0Backup1W1Spec>;
#[doc = "eFuse apb2otp block0 data register2."]
pub mod apb2otp_blk0_backup1_w1;
#[doc = "APB2OTP_BLK0_BACKUP1_W2 (r) register accessor: eFuse apb2otp block0 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup1_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup1_w2`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP1_W2")]
pub type Apb2otpBlk0Backup1W2 = crate::Reg<apb2otp_blk0_backup1_w2::Apb2otpBlk0Backup1W2Spec>;
#[doc = "eFuse apb2otp block0 data register3."]
pub mod apb2otp_blk0_backup1_w2;
#[doc = "APB2OTP_BLK0_BACKUP1_W3 (r) register accessor: eFuse apb2otp block0 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup1_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup1_w3`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP1_W3")]
pub type Apb2otpBlk0Backup1W3 = crate::Reg<apb2otp_blk0_backup1_w3::Apb2otpBlk0Backup1W3Spec>;
#[doc = "eFuse apb2otp block0 data register4."]
pub mod apb2otp_blk0_backup1_w3;
#[doc = "APB2OTP_BLK0_BACKUP1_W4 (r) register accessor: eFuse apb2otp block0 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup1_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup1_w4`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP1_W4")]
pub type Apb2otpBlk0Backup1W4 = crate::Reg<apb2otp_blk0_backup1_w4::Apb2otpBlk0Backup1W4Spec>;
#[doc = "eFuse apb2otp block0 data register5."]
pub mod apb2otp_blk0_backup1_w4;
#[doc = "APB2OTP_BLK0_BACKUP1_W5 (r) register accessor: eFuse apb2otp block0 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup1_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup1_w5`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP1_W5")]
pub type Apb2otpBlk0Backup1W5 = crate::Reg<apb2otp_blk0_backup1_w5::Apb2otpBlk0Backup1W5Spec>;
#[doc = "eFuse apb2otp block0 data register6."]
pub mod apb2otp_blk0_backup1_w5;
#[doc = "APB2OTP_BLK0_BACKUP2_W1 (r) register accessor: eFuse apb2otp block0 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup2_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup2_w1`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP2_W1")]
pub type Apb2otpBlk0Backup2W1 = crate::Reg<apb2otp_blk0_backup2_w1::Apb2otpBlk0Backup2W1Spec>;
#[doc = "eFuse apb2otp block0 data register7."]
pub mod apb2otp_blk0_backup2_w1;
#[doc = "APB2OTP_BLK0_BACKUP2_W2 (r) register accessor: eFuse apb2otp block0 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup2_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup2_w2`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP2_W2")]
pub type Apb2otpBlk0Backup2W2 = crate::Reg<apb2otp_blk0_backup2_w2::Apb2otpBlk0Backup2W2Spec>;
#[doc = "eFuse apb2otp block0 data register8."]
pub mod apb2otp_blk0_backup2_w2;
#[doc = "APB2OTP_BLK0_BACKUP2_W3 (r) register accessor: eFuse apb2otp block0 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup2_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup2_w3`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP2_W3")]
pub type Apb2otpBlk0Backup2W3 = crate::Reg<apb2otp_blk0_backup2_w3::Apb2otpBlk0Backup2W3Spec>;
#[doc = "eFuse apb2otp block0 data register9."]
pub mod apb2otp_blk0_backup2_w3;
#[doc = "APB2OTP_BLK0_BACKUP2_W4 (r) register accessor: eFuse apb2otp block0 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup2_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup2_w4`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP2_W4")]
pub type Apb2otpBlk0Backup2W4 = crate::Reg<apb2otp_blk0_backup2_w4::Apb2otpBlk0Backup2W4Spec>;
#[doc = "eFuse apb2otp block0 data register10."]
pub mod apb2otp_blk0_backup2_w4;
#[doc = "APB2OTP_BLK0_BACKUP2_W5 (r) register accessor: eFuse apb2otp block0 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup2_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup2_w5`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP2_W5")]
pub type Apb2otpBlk0Backup2W5 = crate::Reg<apb2otp_blk0_backup2_w5::Apb2otpBlk0Backup2W5Spec>;
#[doc = "eFuse apb2otp block0 data register11."]
pub mod apb2otp_blk0_backup2_w5;
#[doc = "APB2OTP_BLK0_BACKUP3_W1 (r) register accessor: eFuse apb2otp block0 data register12.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup3_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup3_w1`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP3_W1")]
pub type Apb2otpBlk0Backup3W1 = crate::Reg<apb2otp_blk0_backup3_w1::Apb2otpBlk0Backup3W1Spec>;
#[doc = "eFuse apb2otp block0 data register12."]
pub mod apb2otp_blk0_backup3_w1;
#[doc = "APB2OTP_BLK0_BACKUP3_W2 (r) register accessor: eFuse apb2otp block0 data register13.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup3_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup3_w2`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP3_W2")]
pub type Apb2otpBlk0Backup3W2 = crate::Reg<apb2otp_blk0_backup3_w2::Apb2otpBlk0Backup3W2Spec>;
#[doc = "eFuse apb2otp block0 data register13."]
pub mod apb2otp_blk0_backup3_w2;
#[doc = "APB2OTP_BLK0_BACKUP3_W3 (r) register accessor: eFuse apb2otp block0 data register14.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup3_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup3_w3`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP3_W3")]
pub type Apb2otpBlk0Backup3W3 = crate::Reg<apb2otp_blk0_backup3_w3::Apb2otpBlk0Backup3W3Spec>;
#[doc = "eFuse apb2otp block0 data register14."]
pub mod apb2otp_blk0_backup3_w3;
#[doc = "APB2OTP_BLK0_BACKUP3_W4 (r) register accessor: eFuse apb2otp block0 data register15.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup3_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup3_w4`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP3_W4")]
pub type Apb2otpBlk0Backup3W4 = crate::Reg<apb2otp_blk0_backup3_w4::Apb2otpBlk0Backup3W4Spec>;
#[doc = "eFuse apb2otp block0 data register15."]
pub mod apb2otp_blk0_backup3_w4;
#[doc = "APB2OTP_BLK0_BACKUP3_W5 (r) register accessor: eFuse apb2otp block0 data register16.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup3_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup3_w5`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP3_W5")]
pub type Apb2otpBlk0Backup3W5 = crate::Reg<apb2otp_blk0_backup3_w5::Apb2otpBlk0Backup3W5Spec>;
#[doc = "eFuse apb2otp block0 data register16."]
pub mod apb2otp_blk0_backup3_w5;
#[doc = "APB2OTP_BLK0_BACKUP4_W1 (r) register accessor: eFuse apb2otp block0 data register17.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup4_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup4_w1`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP4_W1")]
pub type Apb2otpBlk0Backup4W1 = crate::Reg<apb2otp_blk0_backup4_w1::Apb2otpBlk0Backup4W1Spec>;
#[doc = "eFuse apb2otp block0 data register17."]
pub mod apb2otp_blk0_backup4_w1;
#[doc = "APB2OTP_BLK0_BACKUP4_W2 (r) register accessor: eFuse apb2otp block0 data register18.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup4_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup4_w2`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP4_W2")]
pub type Apb2otpBlk0Backup4W2 = crate::Reg<apb2otp_blk0_backup4_w2::Apb2otpBlk0Backup4W2Spec>;
#[doc = "eFuse apb2otp block0 data register18."]
pub mod apb2otp_blk0_backup4_w2;
#[doc = "APB2OTP_BLK0_BACKUP4_W3 (r) register accessor: eFuse apb2otp block0 data register19.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup4_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup4_w3`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP4_W3")]
pub type Apb2otpBlk0Backup4W3 = crate::Reg<apb2otp_blk0_backup4_w3::Apb2otpBlk0Backup4W3Spec>;
#[doc = "eFuse apb2otp block0 data register19."]
pub mod apb2otp_blk0_backup4_w3;
#[doc = "APB2OTP_BLK0_BACKUP4_W4 (r) register accessor: eFuse apb2otp block0 data register20.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup4_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup4_w4`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP4_W4")]
pub type Apb2otpBlk0Backup4W4 = crate::Reg<apb2otp_blk0_backup4_w4::Apb2otpBlk0Backup4W4Spec>;
#[doc = "eFuse apb2otp block0 data register20."]
pub mod apb2otp_blk0_backup4_w4;
#[doc = "APB2OTP_BLK0_BACKUP4_W5 (r) register accessor: eFuse apb2otp block0 data register21.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup4_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup4_w5`] module"]
#[doc(alias = "APB2OTP_BLK0_BACKUP4_W5")]
pub type Apb2otpBlk0Backup4W5 = crate::Reg<apb2otp_blk0_backup4_w5::Apb2otpBlk0Backup4W5Spec>;
#[doc = "eFuse apb2otp block0 data register21."]
pub mod apb2otp_blk0_backup4_w5;
#[doc = "APB2OTP_BLK1_W1 (r) register accessor: eFuse apb2otp block1 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk1_w1`] module"]
#[doc(alias = "APB2OTP_BLK1_W1")]
pub type Apb2otpBlk1W1 = crate::Reg<apb2otp_blk1_w1::Apb2otpBlk1W1Spec>;
#[doc = "eFuse apb2otp block1 data register1."]
pub mod apb2otp_blk1_w1;
#[doc = "APB2OTP_BLK1_W2 (r) register accessor: eFuse apb2otp block1 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk1_w2`] module"]
#[doc(alias = "APB2OTP_BLK1_W2")]
pub type Apb2otpBlk1W2 = crate::Reg<apb2otp_blk1_w2::Apb2otpBlk1W2Spec>;
#[doc = "eFuse apb2otp block1 data register2."]
pub mod apb2otp_blk1_w2;
#[doc = "APB2OTP_BLK1_W3 (r) register accessor: eFuse apb2otp block1 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk1_w3`] module"]
#[doc(alias = "APB2OTP_BLK1_W3")]
pub type Apb2otpBlk1W3 = crate::Reg<apb2otp_blk1_w3::Apb2otpBlk1W3Spec>;
#[doc = "eFuse apb2otp block1 data register3."]
pub mod apb2otp_blk1_w3;
#[doc = "APB2OTP_BLK1_W4 (r) register accessor: eFuse apb2otp block1 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk1_w4`] module"]
#[doc(alias = "APB2OTP_BLK1_W4")]
pub type Apb2otpBlk1W4 = crate::Reg<apb2otp_blk1_w4::Apb2otpBlk1W4Spec>;
#[doc = "eFuse apb2otp block1 data register4."]
pub mod apb2otp_blk1_w4;
#[doc = "APB2OTP_BLK1_W5 (r) register accessor: eFuse apb2otp block1 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk1_w5`] module"]
#[doc(alias = "APB2OTP_BLK1_W5")]
pub type Apb2otpBlk1W5 = crate::Reg<apb2otp_blk1_w5::Apb2otpBlk1W5Spec>;
#[doc = "eFuse apb2otp block1 data register5."]
pub mod apb2otp_blk1_w5;
#[doc = "APB2OTP_BLK1_W6 (r) register accessor: eFuse apb2otp block1 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk1_w6`] module"]
#[doc(alias = "APB2OTP_BLK1_W6")]
pub type Apb2otpBlk1W6 = crate::Reg<apb2otp_blk1_w6::Apb2otpBlk1W6Spec>;
#[doc = "eFuse apb2otp block1 data register6."]
pub mod apb2otp_blk1_w6;
#[doc = "APB2OTP_BLK1_W7 (r) register accessor: eFuse apb2otp block1 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk1_w7`] module"]
#[doc(alias = "APB2OTP_BLK1_W7")]
pub type Apb2otpBlk1W7 = crate::Reg<apb2otp_blk1_w7::Apb2otpBlk1W7Spec>;
#[doc = "eFuse apb2otp block1 data register7."]
pub mod apb2otp_blk1_w7;
#[doc = "APB2OTP_BLK1_W8 (r) register accessor: eFuse apb2otp block1 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk1_w8`] module"]
#[doc(alias = "APB2OTP_BLK1_W8")]
pub type Apb2otpBlk1W8 = crate::Reg<apb2otp_blk1_w8::Apb2otpBlk1W8Spec>;
#[doc = "eFuse apb2otp block1 data register8."]
pub mod apb2otp_blk1_w8;
#[doc = "APB2OTP_BLK1_W9 (r) register accessor: eFuse apb2otp block1 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk1_w9`] module"]
#[doc(alias = "APB2OTP_BLK1_W9")]
pub type Apb2otpBlk1W9 = crate::Reg<apb2otp_blk1_w9::Apb2otpBlk1W9Spec>;
#[doc = "eFuse apb2otp block1 data register9."]
pub mod apb2otp_blk1_w9;
#[doc = "APB2OTP_BLK2_W1 (r) register accessor: eFuse apb2otp block2 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w1`] module"]
#[doc(alias = "APB2OTP_BLK2_W1")]
pub type Apb2otpBlk2W1 = crate::Reg<apb2otp_blk2_w1::Apb2otpBlk2W1Spec>;
#[doc = "eFuse apb2otp block2 data register1."]
pub mod apb2otp_blk2_w1;
#[doc = "APB2OTP_BLK2_W2 (r) register accessor: eFuse apb2otp block2 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w2`] module"]
#[doc(alias = "APB2OTP_BLK2_W2")]
pub type Apb2otpBlk2W2 = crate::Reg<apb2otp_blk2_w2::Apb2otpBlk2W2Spec>;
#[doc = "eFuse apb2otp block2 data register2."]
pub mod apb2otp_blk2_w2;
#[doc = "APB2OTP_BLK2_W3 (r) register accessor: eFuse apb2otp block2 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w3`] module"]
#[doc(alias = "APB2OTP_BLK2_W3")]
pub type Apb2otpBlk2W3 = crate::Reg<apb2otp_blk2_w3::Apb2otpBlk2W3Spec>;
#[doc = "eFuse apb2otp block2 data register3."]
pub mod apb2otp_blk2_w3;
#[doc = "APB2OTP_BLK2_W4 (r) register accessor: eFuse apb2otp block2 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w4`] module"]
#[doc(alias = "APB2OTP_BLK2_W4")]
pub type Apb2otpBlk2W4 = crate::Reg<apb2otp_blk2_w4::Apb2otpBlk2W4Spec>;
#[doc = "eFuse apb2otp block2 data register4."]
pub mod apb2otp_blk2_w4;
#[doc = "APB2OTP_BLK2_W5 (r) register accessor: eFuse apb2otp block2 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w5`] module"]
#[doc(alias = "APB2OTP_BLK2_W5")]
pub type Apb2otpBlk2W5 = crate::Reg<apb2otp_blk2_w5::Apb2otpBlk2W5Spec>;
#[doc = "eFuse apb2otp block2 data register5."]
pub mod apb2otp_blk2_w5;
#[doc = "APB2OTP_BLK2_W6 (r) register accessor: eFuse apb2otp block2 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w6`] module"]
#[doc(alias = "APB2OTP_BLK2_W6")]
pub type Apb2otpBlk2W6 = crate::Reg<apb2otp_blk2_w6::Apb2otpBlk2W6Spec>;
#[doc = "eFuse apb2otp block2 data register6."]
pub mod apb2otp_blk2_w6;
#[doc = "APB2OTP_BLK2_W7 (r) register accessor: eFuse apb2otp block2 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w7`] module"]
#[doc(alias = "APB2OTP_BLK2_W7")]
pub type Apb2otpBlk2W7 = crate::Reg<apb2otp_blk2_w7::Apb2otpBlk2W7Spec>;
#[doc = "eFuse apb2otp block2 data register7."]
pub mod apb2otp_blk2_w7;
#[doc = "APB2OTP_BLK2_W8 (r) register accessor: eFuse apb2otp block2 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w8`] module"]
#[doc(alias = "APB2OTP_BLK2_W8")]
pub type Apb2otpBlk2W8 = crate::Reg<apb2otp_blk2_w8::Apb2otpBlk2W8Spec>;
#[doc = "eFuse apb2otp block2 data register8."]
pub mod apb2otp_blk2_w8;
#[doc = "APB2OTP_BLK2_W9 (r) register accessor: eFuse apb2otp block2 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w9`] module"]
#[doc(alias = "APB2OTP_BLK2_W9")]
pub type Apb2otpBlk2W9 = crate::Reg<apb2otp_blk2_w9::Apb2otpBlk2W9Spec>;
#[doc = "eFuse apb2otp block2 data register9."]
pub mod apb2otp_blk2_w9;
#[doc = "APB2OTP_BLK2_W10 (r) register accessor: eFuse apb2otp block2 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w10`] module"]
#[doc(alias = "APB2OTP_BLK2_W10")]
pub type Apb2otpBlk2W10 = crate::Reg<apb2otp_blk2_w10::Apb2otpBlk2W10Spec>;
#[doc = "eFuse apb2otp block2 data register10."]
pub mod apb2otp_blk2_w10;
#[doc = "APB2OTP_BLK2_W11 (r) register accessor: eFuse apb2otp block2 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w11`] module"]
#[doc(alias = "APB2OTP_BLK2_W11")]
pub type Apb2otpBlk2W11 = crate::Reg<apb2otp_blk2_w11::Apb2otpBlk2W11Spec>;
#[doc = "eFuse apb2otp block2 data register11."]
pub mod apb2otp_blk2_w11;
#[doc = "APB2OTP_BLK3_W1 (r) register accessor: eFuse apb2otp block3 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w1`] module"]
#[doc(alias = "APB2OTP_BLK3_W1")]
pub type Apb2otpBlk3W1 = crate::Reg<apb2otp_blk3_w1::Apb2otpBlk3W1Spec>;
#[doc = "eFuse apb2otp block3 data register1."]
pub mod apb2otp_blk3_w1;
#[doc = "APB2OTP_BLK3_W2 (r) register accessor: eFuse apb2otp block3 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w2`] module"]
#[doc(alias = "APB2OTP_BLK3_W2")]
pub type Apb2otpBlk3W2 = crate::Reg<apb2otp_blk3_w2::Apb2otpBlk3W2Spec>;
#[doc = "eFuse apb2otp block3 data register2."]
pub mod apb2otp_blk3_w2;
#[doc = "APB2OTP_BLK3_W3 (r) register accessor: eFuse apb2otp block3 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w3`] module"]
#[doc(alias = "APB2OTP_BLK3_W3")]
pub type Apb2otpBlk3W3 = crate::Reg<apb2otp_blk3_w3::Apb2otpBlk3W3Spec>;
#[doc = "eFuse apb2otp block3 data register3."]
pub mod apb2otp_blk3_w3;
#[doc = "APB2OTP_BLK3_W4 (r) register accessor: eFuse apb2otp block3 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w4`] module"]
#[doc(alias = "APB2OTP_BLK3_W4")]
pub type Apb2otpBlk3W4 = crate::Reg<apb2otp_blk3_w4::Apb2otpBlk3W4Spec>;
#[doc = "eFuse apb2otp block3 data register4."]
pub mod apb2otp_blk3_w4;
#[doc = "APB2OTP_BLK3_W5 (r) register accessor: eFuse apb2otp block3 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w5`] module"]
#[doc(alias = "APB2OTP_BLK3_W5")]
pub type Apb2otpBlk3W5 = crate::Reg<apb2otp_blk3_w5::Apb2otpBlk3W5Spec>;
#[doc = "eFuse apb2otp block3 data register5."]
pub mod apb2otp_blk3_w5;
#[doc = "APB2OTP_BLK3_W6 (r) register accessor: eFuse apb2otp block3 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w6`] module"]
#[doc(alias = "APB2OTP_BLK3_W6")]
pub type Apb2otpBlk3W6 = crate::Reg<apb2otp_blk3_w6::Apb2otpBlk3W6Spec>;
#[doc = "eFuse apb2otp block3 data register6."]
pub mod apb2otp_blk3_w6;
#[doc = "APB2OTP_BLK3_W7 (r) register accessor: eFuse apb2otp block3 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w7`] module"]
#[doc(alias = "APB2OTP_BLK3_W7")]
pub type Apb2otpBlk3W7 = crate::Reg<apb2otp_blk3_w7::Apb2otpBlk3W7Spec>;
#[doc = "eFuse apb2otp block3 data register7."]
pub mod apb2otp_blk3_w7;
#[doc = "APB2OTP_BLK3_W8 (r) register accessor: eFuse apb2otp block3 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w8`] module"]
#[doc(alias = "APB2OTP_BLK3_W8")]
pub type Apb2otpBlk3W8 = crate::Reg<apb2otp_blk3_w8::Apb2otpBlk3W8Spec>;
#[doc = "eFuse apb2otp block3 data register8."]
pub mod apb2otp_blk3_w8;
#[doc = "APB2OTP_BLK3_W9 (r) register accessor: eFuse apb2otp block3 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w9`] module"]
#[doc(alias = "APB2OTP_BLK3_W9")]
pub type Apb2otpBlk3W9 = crate::Reg<apb2otp_blk3_w9::Apb2otpBlk3W9Spec>;
#[doc = "eFuse apb2otp block3 data register9."]
pub mod apb2otp_blk3_w9;
#[doc = "APB2OTP_BLK3_W10 (r) register accessor: eFuse apb2otp block3 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w10`] module"]
#[doc(alias = "APB2OTP_BLK3_W10")]
pub type Apb2otpBlk3W10 = crate::Reg<apb2otp_blk3_w10::Apb2otpBlk3W10Spec>;
#[doc = "eFuse apb2otp block3 data register10."]
pub mod apb2otp_blk3_w10;
#[doc = "APB2OTP_BLK3_W11 (r) register accessor: eFuse apb2otp block3 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w11`] module"]
#[doc(alias = "APB2OTP_BLK3_W11")]
pub type Apb2otpBlk3W11 = crate::Reg<apb2otp_blk3_w11::Apb2otpBlk3W11Spec>;
#[doc = "eFuse apb2otp block3 data register11."]
pub mod apb2otp_blk3_w11;
#[doc = "APB2OTP_BLK4_W1 (r) register accessor: eFuse apb2otp block4 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w1`] module"]
#[doc(alias = "APB2OTP_BLK4_W1")]
pub type Apb2otpBlk4W1 = crate::Reg<apb2otp_blk4_w1::Apb2otpBlk4W1Spec>;
#[doc = "eFuse apb2otp block4 data register1."]
pub mod apb2otp_blk4_w1;
#[doc = "APB2OTP_BLK4_W2 (r) register accessor: eFuse apb2otp block4 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w2`] module"]
#[doc(alias = "APB2OTP_BLK4_W2")]
pub type Apb2otpBlk4W2 = crate::Reg<apb2otp_blk4_w2::Apb2otpBlk4W2Spec>;
#[doc = "eFuse apb2otp block4 data register2."]
pub mod apb2otp_blk4_w2;
#[doc = "APB2OTP_BLK4_W3 (r) register accessor: eFuse apb2otp block4 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w3`] module"]
#[doc(alias = "APB2OTP_BLK4_W3")]
pub type Apb2otpBlk4W3 = crate::Reg<apb2otp_blk4_w3::Apb2otpBlk4W3Spec>;
#[doc = "eFuse apb2otp block4 data register3."]
pub mod apb2otp_blk4_w3;
#[doc = "APB2OTP_BLK4_W4 (r) register accessor: eFuse apb2otp block4 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w4`] module"]
#[doc(alias = "APB2OTP_BLK4_W4")]
pub type Apb2otpBlk4W4 = crate::Reg<apb2otp_blk4_w4::Apb2otpBlk4W4Spec>;
#[doc = "eFuse apb2otp block4 data register4."]
pub mod apb2otp_blk4_w4;
#[doc = "APB2OTP_BLK4_W5 (r) register accessor: eFuse apb2otp block4 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w5`] module"]
#[doc(alias = "APB2OTP_BLK4_W5")]
pub type Apb2otpBlk4W5 = crate::Reg<apb2otp_blk4_w5::Apb2otpBlk4W5Spec>;
#[doc = "eFuse apb2otp block4 data register5."]
pub mod apb2otp_blk4_w5;
#[doc = "APB2OTP_BLK4_W6 (r) register accessor: eFuse apb2otp block4 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w6`] module"]
#[doc(alias = "APB2OTP_BLK4_W6")]
pub type Apb2otpBlk4W6 = crate::Reg<apb2otp_blk4_w6::Apb2otpBlk4W6Spec>;
#[doc = "eFuse apb2otp block4 data register6."]
pub mod apb2otp_blk4_w6;
#[doc = "APB2OTP_BLK4_W7 (r) register accessor: eFuse apb2otp block4 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w7`] module"]
#[doc(alias = "APB2OTP_BLK4_W7")]
pub type Apb2otpBlk4W7 = crate::Reg<apb2otp_blk4_w7::Apb2otpBlk4W7Spec>;
#[doc = "eFuse apb2otp block4 data register7."]
pub mod apb2otp_blk4_w7;
#[doc = "APB2OTP_BLK4_W8 (r) register accessor: eFuse apb2otp block4 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w8`] module"]
#[doc(alias = "APB2OTP_BLK4_W8")]
pub type Apb2otpBlk4W8 = crate::Reg<apb2otp_blk4_w8::Apb2otpBlk4W8Spec>;
#[doc = "eFuse apb2otp block4 data register8."]
pub mod apb2otp_blk4_w8;
#[doc = "APB2OTP_BLK4_W9 (r) register accessor: eFuse apb2otp block4 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w9`] module"]
#[doc(alias = "APB2OTP_BLK4_W9")]
pub type Apb2otpBlk4W9 = crate::Reg<apb2otp_blk4_w9::Apb2otpBlk4W9Spec>;
#[doc = "eFuse apb2otp block4 data register9."]
pub mod apb2otp_blk4_w9;
#[doc = "APB2OTP_BLK4_W10 (r) register accessor: eFuse apb2otp block4 data registe10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w10`] module"]
#[doc(alias = "APB2OTP_BLK4_W10")]
pub type Apb2otpBlk4W10 = crate::Reg<apb2otp_blk4_w10::Apb2otpBlk4W10Spec>;
#[doc = "eFuse apb2otp block4 data registe10."]
pub mod apb2otp_blk4_w10;
#[doc = "APB2OTP_BLK4_W11 (r) register accessor: eFuse apb2otp block4 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w11`] module"]
#[doc(alias = "APB2OTP_BLK4_W11")]
pub type Apb2otpBlk4W11 = crate::Reg<apb2otp_blk4_w11::Apb2otpBlk4W11Spec>;
#[doc = "eFuse apb2otp block4 data register11."]
pub mod apb2otp_blk4_w11;
#[doc = "APB2OTP_BLK5_W1 (r) register accessor: eFuse apb2otp block5 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w1`] module"]
#[doc(alias = "APB2OTP_BLK5_W1")]
pub type Apb2otpBlk5W1 = crate::Reg<apb2otp_blk5_w1::Apb2otpBlk5W1Spec>;
#[doc = "eFuse apb2otp block5 data register1."]
pub mod apb2otp_blk5_w1;
#[doc = "APB2OTP_BLK5_W2 (r) register accessor: eFuse apb2otp block5 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w2`] module"]
#[doc(alias = "APB2OTP_BLK5_W2")]
pub type Apb2otpBlk5W2 = crate::Reg<apb2otp_blk5_w2::Apb2otpBlk5W2Spec>;
#[doc = "eFuse apb2otp block5 data register2."]
pub mod apb2otp_blk5_w2;
#[doc = "APB2OTP_BLK5_W3 (r) register accessor: eFuse apb2otp block5 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w3`] module"]
#[doc(alias = "APB2OTP_BLK5_W3")]
pub type Apb2otpBlk5W3 = crate::Reg<apb2otp_blk5_w3::Apb2otpBlk5W3Spec>;
#[doc = "eFuse apb2otp block5 data register3."]
pub mod apb2otp_blk5_w3;
#[doc = "APB2OTP_BLK5_W4 (r) register accessor: eFuse apb2otp block5 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w4`] module"]
#[doc(alias = "APB2OTP_BLK5_W4")]
pub type Apb2otpBlk5W4 = crate::Reg<apb2otp_blk5_w4::Apb2otpBlk5W4Spec>;
#[doc = "eFuse apb2otp block5 data register4."]
pub mod apb2otp_blk5_w4;
#[doc = "APB2OTP_BLK5_W5 (r) register accessor: eFuse apb2otp block5 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w5`] module"]
#[doc(alias = "APB2OTP_BLK5_W5")]
pub type Apb2otpBlk5W5 = crate::Reg<apb2otp_blk5_w5::Apb2otpBlk5W5Spec>;
#[doc = "eFuse apb2otp block5 data register5."]
pub mod apb2otp_blk5_w5;
#[doc = "APB2OTP_BLK5_W6 (r) register accessor: eFuse apb2otp block5 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w6`] module"]
#[doc(alias = "APB2OTP_BLK5_W6")]
pub type Apb2otpBlk5W6 = crate::Reg<apb2otp_blk5_w6::Apb2otpBlk5W6Spec>;
#[doc = "eFuse apb2otp block5 data register6."]
pub mod apb2otp_blk5_w6;
#[doc = "APB2OTP_BLK5_W7 (r) register accessor: eFuse apb2otp block5 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w7`] module"]
#[doc(alias = "APB2OTP_BLK5_W7")]
pub type Apb2otpBlk5W7 = crate::Reg<apb2otp_blk5_w7::Apb2otpBlk5W7Spec>;
#[doc = "eFuse apb2otp block5 data register7."]
pub mod apb2otp_blk5_w7;
#[doc = "APB2OTP_BLK5_W8 (r) register accessor: eFuse apb2otp block5 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w8`] module"]
#[doc(alias = "APB2OTP_BLK5_W8")]
pub type Apb2otpBlk5W8 = crate::Reg<apb2otp_blk5_w8::Apb2otpBlk5W8Spec>;
#[doc = "eFuse apb2otp block5 data register8."]
pub mod apb2otp_blk5_w8;
#[doc = "APB2OTP_BLK5_W9 (r) register accessor: eFuse apb2otp block5 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w9`] module"]
#[doc(alias = "APB2OTP_BLK5_W9")]
pub type Apb2otpBlk5W9 = crate::Reg<apb2otp_blk5_w9::Apb2otpBlk5W9Spec>;
#[doc = "eFuse apb2otp block5 data register9."]
pub mod apb2otp_blk5_w9;
#[doc = "APB2OTP_BLK5_W10 (r) register accessor: eFuse apb2otp block5 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w10`] module"]
#[doc(alias = "APB2OTP_BLK5_W10")]
pub type Apb2otpBlk5W10 = crate::Reg<apb2otp_blk5_w10::Apb2otpBlk5W10Spec>;
#[doc = "eFuse apb2otp block5 data register10."]
pub mod apb2otp_blk5_w10;
#[doc = "APB2OTP_BLK5_W11 (r) register accessor: eFuse apb2otp block5 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w11`] module"]
#[doc(alias = "APB2OTP_BLK5_W11")]
pub type Apb2otpBlk5W11 = crate::Reg<apb2otp_blk5_w11::Apb2otpBlk5W11Spec>;
#[doc = "eFuse apb2otp block5 data register11."]
pub mod apb2otp_blk5_w11;
#[doc = "APB2OTP_BLK6_W1 (r) register accessor: eFuse apb2otp block6 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w1`] module"]
#[doc(alias = "APB2OTP_BLK6_W1")]
pub type Apb2otpBlk6W1 = crate::Reg<apb2otp_blk6_w1::Apb2otpBlk6W1Spec>;
#[doc = "eFuse apb2otp block6 data register1."]
pub mod apb2otp_blk6_w1;
#[doc = "APB2OTP_BLK6_W2 (r) register accessor: eFuse apb2otp block6 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w2`] module"]
#[doc(alias = "APB2OTP_BLK6_W2")]
pub type Apb2otpBlk6W2 = crate::Reg<apb2otp_blk6_w2::Apb2otpBlk6W2Spec>;
#[doc = "eFuse apb2otp block6 data register2."]
pub mod apb2otp_blk6_w2;
#[doc = "APB2OTP_BLK6_W3 (r) register accessor: eFuse apb2otp block6 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w3`] module"]
#[doc(alias = "APB2OTP_BLK6_W3")]
pub type Apb2otpBlk6W3 = crate::Reg<apb2otp_blk6_w3::Apb2otpBlk6W3Spec>;
#[doc = "eFuse apb2otp block6 data register3."]
pub mod apb2otp_blk6_w3;
#[doc = "APB2OTP_BLK6_W4 (r) register accessor: eFuse apb2otp block6 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w4`] module"]
#[doc(alias = "APB2OTP_BLK6_W4")]
pub type Apb2otpBlk6W4 = crate::Reg<apb2otp_blk6_w4::Apb2otpBlk6W4Spec>;
#[doc = "eFuse apb2otp block6 data register4."]
pub mod apb2otp_blk6_w4;
#[doc = "APB2OTP_BLK6_W5 (r) register accessor: eFuse apb2otp block6 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w5`] module"]
#[doc(alias = "APB2OTP_BLK6_W5")]
pub type Apb2otpBlk6W5 = crate::Reg<apb2otp_blk6_w5::Apb2otpBlk6W5Spec>;
#[doc = "eFuse apb2otp block6 data register5."]
pub mod apb2otp_blk6_w5;
#[doc = "APB2OTP_BLK6_W6 (r) register accessor: eFuse apb2otp block6 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w6`] module"]
#[doc(alias = "APB2OTP_BLK6_W6")]
pub type Apb2otpBlk6W6 = crate::Reg<apb2otp_blk6_w6::Apb2otpBlk6W6Spec>;
#[doc = "eFuse apb2otp block6 data register6."]
pub mod apb2otp_blk6_w6;
#[doc = "APB2OTP_BLK6_W7 (r) register accessor: eFuse apb2otp block6 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w7`] module"]
#[doc(alias = "APB2OTP_BLK6_W7")]
pub type Apb2otpBlk6W7 = crate::Reg<apb2otp_blk6_w7::Apb2otpBlk6W7Spec>;
#[doc = "eFuse apb2otp block6 data register7."]
pub mod apb2otp_blk6_w7;
#[doc = "APB2OTP_BLK6_W8 (r) register accessor: eFuse apb2otp block6 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w8`] module"]
#[doc(alias = "APB2OTP_BLK6_W8")]
pub type Apb2otpBlk6W8 = crate::Reg<apb2otp_blk6_w8::Apb2otpBlk6W8Spec>;
#[doc = "eFuse apb2otp block6 data register8."]
pub mod apb2otp_blk6_w8;
#[doc = "APB2OTP_BLK6_W9 (r) register accessor: eFuse apb2otp block6 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w9`] module"]
#[doc(alias = "APB2OTP_BLK6_W9")]
pub type Apb2otpBlk6W9 = crate::Reg<apb2otp_blk6_w9::Apb2otpBlk6W9Spec>;
#[doc = "eFuse apb2otp block6 data register9."]
pub mod apb2otp_blk6_w9;
#[doc = "APB2OTP_BLK6_W10 (r) register accessor: eFuse apb2otp block6 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w10`] module"]
#[doc(alias = "APB2OTP_BLK6_W10")]
pub type Apb2otpBlk6W10 = crate::Reg<apb2otp_blk6_w10::Apb2otpBlk6W10Spec>;
#[doc = "eFuse apb2otp block6 data register10."]
pub mod apb2otp_blk6_w10;
#[doc = "APB2OTP_BLK6_W11 (r) register accessor: eFuse apb2otp block6 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w11`] module"]
#[doc(alias = "APB2OTP_BLK6_W11")]
pub type Apb2otpBlk6W11 = crate::Reg<apb2otp_blk6_w11::Apb2otpBlk6W11Spec>;
#[doc = "eFuse apb2otp block6 data register11."]
pub mod apb2otp_blk6_w11;
#[doc = "APB2OTP_BLK7_W1 (r) register accessor: eFuse apb2otp block7 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w1`] module"]
#[doc(alias = "APB2OTP_BLK7_W1")]
pub type Apb2otpBlk7W1 = crate::Reg<apb2otp_blk7_w1::Apb2otpBlk7W1Spec>;
#[doc = "eFuse apb2otp block7 data register1."]
pub mod apb2otp_blk7_w1;
#[doc = "APB2OTP_BLK7_W2 (r) register accessor: eFuse apb2otp block7 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w2`] module"]
#[doc(alias = "APB2OTP_BLK7_W2")]
pub type Apb2otpBlk7W2 = crate::Reg<apb2otp_blk7_w2::Apb2otpBlk7W2Spec>;
#[doc = "eFuse apb2otp block7 data register2."]
pub mod apb2otp_blk7_w2;
#[doc = "APB2OTP_BLK7_W3 (r) register accessor: eFuse apb2otp block7 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w3`] module"]
#[doc(alias = "APB2OTP_BLK7_W3")]
pub type Apb2otpBlk7W3 = crate::Reg<apb2otp_blk7_w3::Apb2otpBlk7W3Spec>;
#[doc = "eFuse apb2otp block7 data register3."]
pub mod apb2otp_blk7_w3;
#[doc = "APB2OTP_BLK7_W4 (r) register accessor: eFuse apb2otp block7 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w4`] module"]
#[doc(alias = "APB2OTP_BLK7_W4")]
pub type Apb2otpBlk7W4 = crate::Reg<apb2otp_blk7_w4::Apb2otpBlk7W4Spec>;
#[doc = "eFuse apb2otp block7 data register4."]
pub mod apb2otp_blk7_w4;
#[doc = "APB2OTP_BLK7_W5 (r) register accessor: eFuse apb2otp block7 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w5`] module"]
#[doc(alias = "APB2OTP_BLK7_W5")]
pub type Apb2otpBlk7W5 = crate::Reg<apb2otp_blk7_w5::Apb2otpBlk7W5Spec>;
#[doc = "eFuse apb2otp block7 data register5."]
pub mod apb2otp_blk7_w5;
#[doc = "APB2OTP_BLK7_W6 (r) register accessor: eFuse apb2otp block7 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w6`] module"]
#[doc(alias = "APB2OTP_BLK7_W6")]
pub type Apb2otpBlk7W6 = crate::Reg<apb2otp_blk7_w6::Apb2otpBlk7W6Spec>;
#[doc = "eFuse apb2otp block7 data register6."]
pub mod apb2otp_blk7_w6;
#[doc = "APB2OTP_BLK7_W7 (r) register accessor: eFuse apb2otp block7 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w7`] module"]
#[doc(alias = "APB2OTP_BLK7_W7")]
pub type Apb2otpBlk7W7 = crate::Reg<apb2otp_blk7_w7::Apb2otpBlk7W7Spec>;
#[doc = "eFuse apb2otp block7 data register7."]
pub mod apb2otp_blk7_w7;
#[doc = "APB2OTP_BLK7_W8 (r) register accessor: eFuse apb2otp block7 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w8`] module"]
#[doc(alias = "APB2OTP_BLK7_W8")]
pub type Apb2otpBlk7W8 = crate::Reg<apb2otp_blk7_w8::Apb2otpBlk7W8Spec>;
#[doc = "eFuse apb2otp block7 data register8."]
pub mod apb2otp_blk7_w8;
#[doc = "APB2OTP_BLK7_W9 (r) register accessor: eFuse apb2otp block7 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w9`] module"]
#[doc(alias = "APB2OTP_BLK7_W9")]
pub type Apb2otpBlk7W9 = crate::Reg<apb2otp_blk7_w9::Apb2otpBlk7W9Spec>;
#[doc = "eFuse apb2otp block7 data register9."]
pub mod apb2otp_blk7_w9;
#[doc = "APB2OTP_BLK7_W10 (r) register accessor: eFuse apb2otp block7 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w10`] module"]
#[doc(alias = "APB2OTP_BLK7_W10")]
pub type Apb2otpBlk7W10 = crate::Reg<apb2otp_blk7_w10::Apb2otpBlk7W10Spec>;
#[doc = "eFuse apb2otp block7 data register10."]
pub mod apb2otp_blk7_w10;
#[doc = "APB2OTP_BLK7_W11 (r) register accessor: eFuse apb2otp block7 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w11`] module"]
#[doc(alias = "APB2OTP_BLK7_W11")]
pub type Apb2otpBlk7W11 = crate::Reg<apb2otp_blk7_w11::Apb2otpBlk7W11Spec>;
#[doc = "eFuse apb2otp block7 data register11."]
pub mod apb2otp_blk7_w11;
#[doc = "APB2OTP_BLK8_W1 (r) register accessor: eFuse apb2otp block8 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w1`] module"]
#[doc(alias = "APB2OTP_BLK8_W1")]
pub type Apb2otpBlk8W1 = crate::Reg<apb2otp_blk8_w1::Apb2otpBlk8W1Spec>;
#[doc = "eFuse apb2otp block8 data register1."]
pub mod apb2otp_blk8_w1;
#[doc = "APB2OTP_BLK8_W2 (r) register accessor: eFuse apb2otp block8 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w2`] module"]
#[doc(alias = "APB2OTP_BLK8_W2")]
pub type Apb2otpBlk8W2 = crate::Reg<apb2otp_blk8_w2::Apb2otpBlk8W2Spec>;
#[doc = "eFuse apb2otp block8 data register2."]
pub mod apb2otp_blk8_w2;
#[doc = "APB2OTP_BLK8_W3 (r) register accessor: eFuse apb2otp block8 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w3`] module"]
#[doc(alias = "APB2OTP_BLK8_W3")]
pub type Apb2otpBlk8W3 = crate::Reg<apb2otp_blk8_w3::Apb2otpBlk8W3Spec>;
#[doc = "eFuse apb2otp block8 data register3."]
pub mod apb2otp_blk8_w3;
#[doc = "APB2OTP_BLK8_W4 (r) register accessor: eFuse apb2otp block8 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w4`] module"]
#[doc(alias = "APB2OTP_BLK8_W4")]
pub type Apb2otpBlk8W4 = crate::Reg<apb2otp_blk8_w4::Apb2otpBlk8W4Spec>;
#[doc = "eFuse apb2otp block8 data register4."]
pub mod apb2otp_blk8_w4;
#[doc = "APB2OTP_BLK8_W5 (r) register accessor: eFuse apb2otp block8 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w5`] module"]
#[doc(alias = "APB2OTP_BLK8_W5")]
pub type Apb2otpBlk8W5 = crate::Reg<apb2otp_blk8_w5::Apb2otpBlk8W5Spec>;
#[doc = "eFuse apb2otp block8 data register5."]
pub mod apb2otp_blk8_w5;
#[doc = "APB2OTP_BLK8_W6 (r) register accessor: eFuse apb2otp block8 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w6`] module"]
#[doc(alias = "APB2OTP_BLK8_W6")]
pub type Apb2otpBlk8W6 = crate::Reg<apb2otp_blk8_w6::Apb2otpBlk8W6Spec>;
#[doc = "eFuse apb2otp block8 data register6."]
pub mod apb2otp_blk8_w6;
#[doc = "APB2OTP_BLK8_W7 (r) register accessor: eFuse apb2otp block8 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w7`] module"]
#[doc(alias = "APB2OTP_BLK8_W7")]
pub type Apb2otpBlk8W7 = crate::Reg<apb2otp_blk8_w7::Apb2otpBlk8W7Spec>;
#[doc = "eFuse apb2otp block8 data register7."]
pub mod apb2otp_blk8_w7;
#[doc = "APB2OTP_BLK8_W8 (r) register accessor: eFuse apb2otp block8 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w8`] module"]
#[doc(alias = "APB2OTP_BLK8_W8")]
pub type Apb2otpBlk8W8 = crate::Reg<apb2otp_blk8_w8::Apb2otpBlk8W8Spec>;
#[doc = "eFuse apb2otp block8 data register8."]
pub mod apb2otp_blk8_w8;
#[doc = "APB2OTP_BLK8_W9 (r) register accessor: eFuse apb2otp block8 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w9`] module"]
#[doc(alias = "APB2OTP_BLK8_W9")]
pub type Apb2otpBlk8W9 = crate::Reg<apb2otp_blk8_w9::Apb2otpBlk8W9Spec>;
#[doc = "eFuse apb2otp block8 data register9."]
pub mod apb2otp_blk8_w9;
#[doc = "APB2OTP_BLK8_W10 (r) register accessor: eFuse apb2otp block8 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w10`] module"]
#[doc(alias = "APB2OTP_BLK8_W10")]
pub type Apb2otpBlk8W10 = crate::Reg<apb2otp_blk8_w10::Apb2otpBlk8W10Spec>;
#[doc = "eFuse apb2otp block8 data register10."]
pub mod apb2otp_blk8_w10;
#[doc = "APB2OTP_BLK8_W11 (r) register accessor: eFuse apb2otp block8 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w11`] module"]
#[doc(alias = "APB2OTP_BLK8_W11")]
pub type Apb2otpBlk8W11 = crate::Reg<apb2otp_blk8_w11::Apb2otpBlk8W11Spec>;
#[doc = "eFuse apb2otp block8 data register11."]
pub mod apb2otp_blk8_w11;
#[doc = "APB2OTP_BLK9_W1 (r) register accessor: eFuse apb2otp block9 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w1`] module"]
#[doc(alias = "APB2OTP_BLK9_W1")]
pub type Apb2otpBlk9W1 = crate::Reg<apb2otp_blk9_w1::Apb2otpBlk9W1Spec>;
#[doc = "eFuse apb2otp block9 data register1."]
pub mod apb2otp_blk9_w1;
#[doc = "APB2OTP_BLK9_W2 (r) register accessor: eFuse apb2otp block9 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w2`] module"]
#[doc(alias = "APB2OTP_BLK9_W2")]
pub type Apb2otpBlk9W2 = crate::Reg<apb2otp_blk9_w2::Apb2otpBlk9W2Spec>;
#[doc = "eFuse apb2otp block9 data register2."]
pub mod apb2otp_blk9_w2;
#[doc = "APB2OTP_BLK9_W3 (r) register accessor: eFuse apb2otp block9 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w3`] module"]
#[doc(alias = "APB2OTP_BLK9_W3")]
pub type Apb2otpBlk9W3 = crate::Reg<apb2otp_blk9_w3::Apb2otpBlk9W3Spec>;
#[doc = "eFuse apb2otp block9 data register3."]
pub mod apb2otp_blk9_w3;
#[doc = "APB2OTP_BLK9_W4 (r) register accessor: eFuse apb2otp block9 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w4`] module"]
#[doc(alias = "APB2OTP_BLK9_W4")]
pub type Apb2otpBlk9W4 = crate::Reg<apb2otp_blk9_w4::Apb2otpBlk9W4Spec>;
#[doc = "eFuse apb2otp block9 data register4."]
pub mod apb2otp_blk9_w4;
#[doc = "APB2OTP_BLK9_W5 (r) register accessor: eFuse apb2otp block9 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w5`] module"]
#[doc(alias = "APB2OTP_BLK9_W5")]
pub type Apb2otpBlk9W5 = crate::Reg<apb2otp_blk9_w5::Apb2otpBlk9W5Spec>;
#[doc = "eFuse apb2otp block9 data register5."]
pub mod apb2otp_blk9_w5;
#[doc = "APB2OTP_BLK9_W6 (r) register accessor: eFuse apb2otp block9 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w6`] module"]
#[doc(alias = "APB2OTP_BLK9_W6")]
pub type Apb2otpBlk9W6 = crate::Reg<apb2otp_blk9_w6::Apb2otpBlk9W6Spec>;
#[doc = "eFuse apb2otp block9 data register6."]
pub mod apb2otp_blk9_w6;
#[doc = "APB2OTP_BLK9_W7 (r) register accessor: eFuse apb2otp block9 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w7`] module"]
#[doc(alias = "APB2OTP_BLK9_W7")]
pub type Apb2otpBlk9W7 = crate::Reg<apb2otp_blk9_w7::Apb2otpBlk9W7Spec>;
#[doc = "eFuse apb2otp block9 data register7."]
pub mod apb2otp_blk9_w7;
#[doc = "APB2OTP_BLK9_W8 (r) register accessor: eFuse apb2otp block9 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w8`] module"]
#[doc(alias = "APB2OTP_BLK9_W8")]
pub type Apb2otpBlk9W8 = crate::Reg<apb2otp_blk9_w8::Apb2otpBlk9W8Spec>;
#[doc = "eFuse apb2otp block9 data register8."]
pub mod apb2otp_blk9_w8;
#[doc = "APB2OTP_BLK9_W9 (r) register accessor: eFuse apb2otp block9 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w9`] module"]
#[doc(alias = "APB2OTP_BLK9_W9")]
pub type Apb2otpBlk9W9 = crate::Reg<apb2otp_blk9_w9::Apb2otpBlk9W9Spec>;
#[doc = "eFuse apb2otp block9 data register9."]
pub mod apb2otp_blk9_w9;
#[doc = "APB2OTP_BLK9_W10 (r) register accessor: eFuse apb2otp block9 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w10`] module"]
#[doc(alias = "APB2OTP_BLK9_W10")]
pub type Apb2otpBlk9W10 = crate::Reg<apb2otp_blk9_w10::Apb2otpBlk9W10Spec>;
#[doc = "eFuse apb2otp block9 data register10."]
pub mod apb2otp_blk9_w10;
#[doc = "APB2OTP_BLK9_W11 (r) register accessor: eFuse apb2otp block9 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w11`] module"]
#[doc(alias = "APB2OTP_BLK9_W11")]
pub type Apb2otpBlk9W11 = crate::Reg<apb2otp_blk9_w11::Apb2otpBlk9W11Spec>;
#[doc = "eFuse apb2otp block9 data register11."]
pub mod apb2otp_blk9_w11;
#[doc = "APB2OTP_BLK10_W1 (r) register accessor: eFuse apb2otp block10 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w1`] module"]
#[doc(alias = "APB2OTP_BLK10_W1")]
pub type Apb2otpBlk10W1 = crate::Reg<apb2otp_blk10_w1::Apb2otpBlk10W1Spec>;
#[doc = "eFuse apb2otp block10 data register1."]
pub mod apb2otp_blk10_w1;
#[doc = "APB2OTP_BLK10_W2 (r) register accessor: eFuse apb2otp block10 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w2`] module"]
#[doc(alias = "APB2OTP_BLK10_W2")]
pub type Apb2otpBlk10W2 = crate::Reg<apb2otp_blk10_w2::Apb2otpBlk10W2Spec>;
#[doc = "eFuse apb2otp block10 data register2."]
pub mod apb2otp_blk10_w2;
#[doc = "APB2OTP_BLK10_W3 (r) register accessor: eFuse apb2otp block10 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w3`] module"]
#[doc(alias = "APB2OTP_BLK10_W3")]
pub type Apb2otpBlk10W3 = crate::Reg<apb2otp_blk10_w3::Apb2otpBlk10W3Spec>;
#[doc = "eFuse apb2otp block10 data register3."]
pub mod apb2otp_blk10_w3;
#[doc = "APB2OTP_BLK10_W4 (r) register accessor: eFuse apb2otp block10 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w4`] module"]
#[doc(alias = "APB2OTP_BLK10_W4")]
pub type Apb2otpBlk10W4 = crate::Reg<apb2otp_blk10_w4::Apb2otpBlk10W4Spec>;
#[doc = "eFuse apb2otp block10 data register4."]
pub mod apb2otp_blk10_w4;
#[doc = "APB2OTP_BLK10_W5 (r) register accessor: eFuse apb2otp block10 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w5`] module"]
#[doc(alias = "APB2OTP_BLK10_W5")]
pub type Apb2otpBlk10W5 = crate::Reg<apb2otp_blk10_w5::Apb2otpBlk10W5Spec>;
#[doc = "eFuse apb2otp block10 data register5."]
pub mod apb2otp_blk10_w5;
#[doc = "APB2OTP_BLK10_W6 (r) register accessor: eFuse apb2otp block10 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w6`] module"]
#[doc(alias = "APB2OTP_BLK10_W6")]
pub type Apb2otpBlk10W6 = crate::Reg<apb2otp_blk10_w6::Apb2otpBlk10W6Spec>;
#[doc = "eFuse apb2otp block10 data register6."]
pub mod apb2otp_blk10_w6;
#[doc = "APB2OTP_BLK10_W7 (r) register accessor: eFuse apb2otp block10 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w7`] module"]
#[doc(alias = "APB2OTP_BLK10_W7")]
pub type Apb2otpBlk10W7 = crate::Reg<apb2otp_blk10_w7::Apb2otpBlk10W7Spec>;
#[doc = "eFuse apb2otp block10 data register7."]
pub mod apb2otp_blk10_w7;
#[doc = "APB2OTP_BLK10_W8 (r) register accessor: eFuse apb2otp block10 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w8`] module"]
#[doc(alias = "APB2OTP_BLK10_W8")]
pub type Apb2otpBlk10W8 = crate::Reg<apb2otp_blk10_w8::Apb2otpBlk10W8Spec>;
#[doc = "eFuse apb2otp block10 data register8."]
pub mod apb2otp_blk10_w8;
#[doc = "APB2OTP_BLK10_W9 (r) register accessor: eFuse apb2otp block10 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w9`] module"]
#[doc(alias = "APB2OTP_BLK10_W9")]
pub type Apb2otpBlk10W9 = crate::Reg<apb2otp_blk10_w9::Apb2otpBlk10W9Spec>;
#[doc = "eFuse apb2otp block10 data register9."]
pub mod apb2otp_blk10_w9;
#[doc = "APB2OTP_BLK10_W10 (r) register accessor: eFuse apb2otp block10 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w10`] module"]
#[doc(alias = "APB2OTP_BLK10_W10")]
pub type Apb2otpBlk10W10 = crate::Reg<apb2otp_blk10_w10::Apb2otpBlk10W10Spec>;
#[doc = "eFuse apb2otp block10 data register10."]
pub mod apb2otp_blk10_w10;
#[doc = "APB2OTP_BLK10_W11 (r) register accessor: eFuse apb2otp block10 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w11`] module"]
#[doc(alias = "APB2OTP_BLK10_W11")]
pub type Apb2otpBlk10W11 = crate::Reg<apb2otp_blk10_w11::Apb2otpBlk10W11Spec>;
#[doc = "eFuse apb2otp block10 data register11."]
pub mod apb2otp_blk10_w11;
#[doc = "APB2OTP_EN (rw) register accessor: eFuse apb2otp enable configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2otp_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_en`] module"]
#[doc(alias = "APB2OTP_EN")]
pub type Apb2otpEn = crate::Reg<apb2otp_en::Apb2otpEnSpec>;
#[doc = "eFuse apb2otp enable configuration register."]
pub mod apb2otp_en;
