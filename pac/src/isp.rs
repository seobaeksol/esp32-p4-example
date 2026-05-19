#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ver_date: VerDate,
    clk_en: ClkEn,
    cntl: Cntl,
    hsync_cnt: HsyncCnt,
    frame_cfg: FrameCfg,
    ccm_coef0: CcmCoef0,
    ccm_coef1: CcmCoef1,
    ccm_coef3: CcmCoef3,
    ccm_coef4: CcmCoef4,
    ccm_coef5: CcmCoef5,
    bf_matrix_ctrl: BfMatrixCtrl,
    bf_sigma: BfSigma,
    bf_gau0: BfGau0,
    bf_gau1: BfGau1,
    dpc_ctrl: DpcCtrl,
    dpc_conf: DpcConf,
    dpc_matrix_ctrl: DpcMatrixCtrl,
    dpc_deadpix_cnt: DpcDeadpixCnt,
    lut_cmd: LutCmd,
    lut_wdata: LutWdata,
    lut_rdata: LutRdata,
    lsc_tablesize: LscTablesize,
    demosaic_matrix_ctrl: DemosaicMatrixCtrl,
    demosaic_grad_ratio: DemosaicGradRatio,
    median_matrix_ctrl: MedianMatrixCtrl,
    int_raw: IntRaw,
    int_st: IntSt,
    int_ena: IntEna,
    int_clr: IntClr,
    gamma_ctrl: GammaCtrl,
    gamma_ry1: GammaRy1,
    gamma_ry2: GammaRy2,
    gamma_ry3: GammaRy3,
    gamma_ry4: GammaRy4,
    gamma_gy1: GammaGy1,
    gamma_gy2: GammaGy2,
    gamma_gy3: GammaGy3,
    gamma_gy4: GammaGy4,
    gamma_by1: GammaBy1,
    gamma_by2: GammaBy2,
    gamma_by3: GammaBy3,
    gamma_by4: GammaBy4,
    gamma_rx1: GammaRx1,
    gamma_rx2: GammaRx2,
    gamma_gx1: GammaGx1,
    gamma_gx2: GammaGx2,
    gamma_bx1: GammaBx1,
    gamma_bx2: GammaBx2,
    ae_ctrl: AeCtrl,
    ae_monitor: AeMonitor,
    ae_bx: AeBx,
    ae_by: AeBy,
    ae_winpixnum: AeWinpixnum,
    ae_win_reciprocal: AeWinReciprocal,
    ae_block_mean_0: AeBlockMean0,
    ae_block_mean_1: AeBlockMean1,
    ae_block_mean_2: AeBlockMean2,
    ae_block_mean_3: AeBlockMean3,
    ae_block_mean_4: AeBlockMean4,
    ae_block_mean_5: AeBlockMean5,
    ae_block_mean_6: AeBlockMean6,
    sharp_ctrl0: SharpCtrl0,
    sharp_filter0: SharpFilter0,
    sharp_filter1: SharpFilter1,
    sharp_filter2: SharpFilter2,
    sharp_matrix_ctrl: SharpMatrixCtrl,
    sharp_ctrl1: SharpCtrl1,
    dma_cntl: DmaCntl,
    dma_raw_data: DmaRawData,
    cam_cntl: CamCntl,
    cam_conf: CamConf,
    af_ctrl0: AfCtrl0,
    af_ctrl1: AfCtrl1,
    af_gen_th_ctrl: AfGenThCtrl,
    af_env_user_th_sum: AfEnvUserThSum,
    af_env_user_th_lum: AfEnvUserThLum,
    af_threshold: AfThreshold,
    af_hscale_a: AfHscaleA,
    af_vscale_a: AfVscaleA,
    af_hscale_b: AfHscaleB,
    af_vscale_b: AfVscaleB,
    af_hscale_c: AfHscaleC,
    af_vscale_c: AfVscaleC,
    af_sum_a: AfSumA,
    af_sum_b: AfSumB,
    af_sum_c: AfSumC,
    af_lum_a: AfLumA,
    af_lum_b: AfLumB,
    af_lum_c: AfLumC,
    awb_mode: AwbMode,
    awb_hscale: AwbHscale,
    awb_vscale: AwbVscale,
    awb_th_lum: AwbThLum,
    awb_th_rg: AwbThRg,
    awb_th_bg: AwbThBg,
    awb0_white_cnt: Awb0WhiteCnt,
    awb0_acc_r: Awb0AccR,
    awb0_acc_g: Awb0AccG,
    awb0_acc_b: Awb0AccB,
    color_ctrl: ColorCtrl,
    blc_value: BlcValue,
    blc_ctrl0: BlcCtrl0,
    blc_ctrl1: BlcCtrl1,
    blc_ctrl2: BlcCtrl2,
    blc_mean: BlcMean,
    hist_mode: HistMode,
    hist_coeff: HistCoeff,
    hist_offs: HistOffs,
    hist_size: HistSize,
    hist_seg0: HistSeg0,
    hist_seg1: HistSeg1,
    hist_seg2: HistSeg2,
    hist_seg3: HistSeg3,
    hist_weight0: HistWeight0,
    hist_weight1: HistWeight1,
    hist_weight2: HistWeight2,
    hist_weight3: HistWeight3,
    hist_weight4: HistWeight4,
    hist_weight5: HistWeight5,
    hist_weight6: HistWeight6,
    hist_bin0: HistBin0,
    hist_bin1: HistBin1,
    hist_bin2: HistBin2,
    hist_bin3: HistBin3,
    hist_bin4: HistBin4,
    hist_bin5: HistBin5,
    hist_bin6: HistBin6,
    hist_bin7: HistBin7,
    hist_bin8: HistBin8,
    hist_bin9: HistBin9,
    hist_bin10: HistBin10,
    hist_bin11: HistBin11,
    hist_bin12: HistBin12,
    hist_bin13: HistBin13,
    hist_bin14: HistBin14,
    hist_bin15: HistBin15,
    mem_aux_ctrl_0: MemAuxCtrl0,
    mem_aux_ctrl_1: MemAuxCtrl1,
    mem_aux_ctrl_2: MemAuxCtrl2,
    mem_aux_ctrl_3: MemAuxCtrl3,
    mem_aux_ctrl_4: MemAuxCtrl4,
    yuv_format: YuvFormat,
    rdn_eco_cs: RdnEcoCs,
    rdn_eco_low: RdnEcoLow,
    rdn_eco_high: RdnEcoHigh,
    crop_ctrl: CropCtrl,
    crop_y_capture: CropYCapture,
    crop_x_capture: CropXCapture,
    crop_err_st: CropErrSt,
    wbg_coef_r: WbgCoefR,
    wbg_coef_g: WbgCoefG,
    wbg_coef_b: WbgCoefB,
    color_hue_ctrl: ColorHueCtrl,
    awb_bx: AwbBx,
    awb_by: AwbBy,
    state: State,
    shadow_reg_ctrl: ShadowRegCtrl,
}
impl RegisterBlock {
    #[doc = "0x00 - version control register"]
    #[inline(always)]
    pub const fn ver_date(&self) -> &VerDate {
        &self.ver_date
    }
    #[doc = "0x04 - isp clk control register"]
    #[inline(always)]
    pub const fn clk_en(&self) -> &ClkEn {
        &self.clk_en
    }
    #[doc = "0x08 - isp module enable control register"]
    #[inline(always)]
    pub const fn cntl(&self) -> &Cntl {
        &self.cntl
    }
    #[doc = "0x0c - header hsync interval control register"]
    #[inline(always)]
    pub const fn hsync_cnt(&self) -> &HsyncCnt {
        &self.hsync_cnt
    }
    #[doc = "0x10 - frame control parameter register"]
    #[inline(always)]
    pub const fn frame_cfg(&self) -> &FrameCfg {
        &self.frame_cfg
    }
    #[doc = "0x14 - ccm coef register 0"]
    #[inline(always)]
    pub const fn ccm_coef0(&self) -> &CcmCoef0 {
        &self.ccm_coef0
    }
    #[doc = "0x18 - ccm coef register 1"]
    #[inline(always)]
    pub const fn ccm_coef1(&self) -> &CcmCoef1 {
        &self.ccm_coef1
    }
    #[doc = "0x1c - ccm coef register 3"]
    #[inline(always)]
    pub const fn ccm_coef3(&self) -> &CcmCoef3 {
        &self.ccm_coef3
    }
    #[doc = "0x20 - ccm coef register 4"]
    #[inline(always)]
    pub const fn ccm_coef4(&self) -> &CcmCoef4 {
        &self.ccm_coef4
    }
    #[doc = "0x24 - ccm coef register 5"]
    #[inline(always)]
    pub const fn ccm_coef5(&self) -> &CcmCoef5 {
        &self.ccm_coef5
    }
    #[doc = "0x28 - bf pix2matrix ctrl"]
    #[inline(always)]
    pub const fn bf_matrix_ctrl(&self) -> &BfMatrixCtrl {
        &self.bf_matrix_ctrl
    }
    #[doc = "0x2c - bf denoising level control register"]
    #[inline(always)]
    pub const fn bf_sigma(&self) -> &BfSigma {
        &self.bf_sigma
    }
    #[doc = "0x30 - bf gau template register 0"]
    #[inline(always)]
    pub const fn bf_gau0(&self) -> &BfGau0 {
        &self.bf_gau0
    }
    #[doc = "0x34 - bf gau template register 1"]
    #[inline(always)]
    pub const fn bf_gau1(&self) -> &BfGau1 {
        &self.bf_gau1
    }
    #[doc = "0x38 - DPC mode control register"]
    #[inline(always)]
    pub const fn dpc_ctrl(&self) -> &DpcCtrl {
        &self.dpc_ctrl
    }
    #[doc = "0x3c - DPC parameter config register"]
    #[inline(always)]
    pub const fn dpc_conf(&self) -> &DpcConf {
        &self.dpc_conf
    }
    #[doc = "0x40 - dpc pix2matrix ctrl"]
    #[inline(always)]
    pub const fn dpc_matrix_ctrl(&self) -> &DpcMatrixCtrl {
        &self.dpc_matrix_ctrl
    }
    #[doc = "0x44 - DPC dead-pix number register"]
    #[inline(always)]
    pub const fn dpc_deadpix_cnt(&self) -> &DpcDeadpixCnt {
        &self.dpc_deadpix_cnt
    }
    #[doc = "0x48 - LUT command register"]
    #[inline(always)]
    pub const fn lut_cmd(&self) -> &LutCmd {
        &self.lut_cmd
    }
    #[doc = "0x4c - LUT write data register"]
    #[inline(always)]
    pub const fn lut_wdata(&self) -> &LutWdata {
        &self.lut_wdata
    }
    #[doc = "0x50 - LUT read data register"]
    #[inline(always)]
    pub const fn lut_rdata(&self) -> &LutRdata {
        &self.lut_rdata
    }
    #[doc = "0x54 - LSC point in x-direction"]
    #[inline(always)]
    pub const fn lsc_tablesize(&self) -> &LscTablesize {
        &self.lsc_tablesize
    }
    #[doc = "0x58 - demosaic pix2matrix ctrl"]
    #[inline(always)]
    pub const fn demosaic_matrix_ctrl(&self) -> &DemosaicMatrixCtrl {
        &self.demosaic_matrix_ctrl
    }
    #[doc = "0x5c - demosaic gradient select ratio"]
    #[inline(always)]
    pub const fn demosaic_grad_ratio(&self) -> &DemosaicGradRatio {
        &self.demosaic_grad_ratio
    }
    #[doc = "0x60 - median pix2matrix ctrl"]
    #[inline(always)]
    pub const fn median_matrix_ctrl(&self) -> &MedianMatrixCtrl {
        &self.median_matrix_ctrl
    }
    #[doc = "0x64 - raw interrupt register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0x68 - masked interrupt register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0x6c - interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0x70 - interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0x74 - gamma control register"]
    #[inline(always)]
    pub const fn gamma_ctrl(&self) -> &GammaCtrl {
        &self.gamma_ctrl
    }
    #[doc = "0x78 - point of Y-axis of r channel gamma curve register 1"]
    #[inline(always)]
    pub const fn gamma_ry1(&self) -> &GammaRy1 {
        &self.gamma_ry1
    }
    #[doc = "0x7c - point of Y-axis of r channel gamma curve register 2"]
    #[inline(always)]
    pub const fn gamma_ry2(&self) -> &GammaRy2 {
        &self.gamma_ry2
    }
    #[doc = "0x80 - point of Y-axis of r channel gamma curve register 3"]
    #[inline(always)]
    pub const fn gamma_ry3(&self) -> &GammaRy3 {
        &self.gamma_ry3
    }
    #[doc = "0x84 - point of Y-axis of r channel gamma curve register 4"]
    #[inline(always)]
    pub const fn gamma_ry4(&self) -> &GammaRy4 {
        &self.gamma_ry4
    }
    #[doc = "0x88 - point of Y-axis of g channel gamma curve register 1"]
    #[inline(always)]
    pub const fn gamma_gy1(&self) -> &GammaGy1 {
        &self.gamma_gy1
    }
    #[doc = "0x8c - point of Y-axis of g channel gamma curve register 2"]
    #[inline(always)]
    pub const fn gamma_gy2(&self) -> &GammaGy2 {
        &self.gamma_gy2
    }
    #[doc = "0x90 - point of Y-axis of g channel gamma curve register 3"]
    #[inline(always)]
    pub const fn gamma_gy3(&self) -> &GammaGy3 {
        &self.gamma_gy3
    }
    #[doc = "0x94 - point of Y-axis of g channel gamma curve register 4"]
    #[inline(always)]
    pub const fn gamma_gy4(&self) -> &GammaGy4 {
        &self.gamma_gy4
    }
    #[doc = "0x98 - point of Y-axis of b channel gamma curve register 1"]
    #[inline(always)]
    pub const fn gamma_by1(&self) -> &GammaBy1 {
        &self.gamma_by1
    }
    #[doc = "0x9c - point of Y-axis of b channel gamma curve register 2"]
    #[inline(always)]
    pub const fn gamma_by2(&self) -> &GammaBy2 {
        &self.gamma_by2
    }
    #[doc = "0xa0 - point of Y-axis of b channel gamma curve register 3"]
    #[inline(always)]
    pub const fn gamma_by3(&self) -> &GammaBy3 {
        &self.gamma_by3
    }
    #[doc = "0xa4 - point of Y-axis of b channel gamma curve register 4"]
    #[inline(always)]
    pub const fn gamma_by4(&self) -> &GammaBy4 {
        &self.gamma_by4
    }
    #[doc = "0xa8 - point of X-axis of r channel gamma curve register 1"]
    #[inline(always)]
    pub const fn gamma_rx1(&self) -> &GammaRx1 {
        &self.gamma_rx1
    }
    #[doc = "0xac - point of X-axis of r channel gamma curve register 2"]
    #[inline(always)]
    pub const fn gamma_rx2(&self) -> &GammaRx2 {
        &self.gamma_rx2
    }
    #[doc = "0xb0 - point of X-axis of g channel gamma curve register 1"]
    #[inline(always)]
    pub const fn gamma_gx1(&self) -> &GammaGx1 {
        &self.gamma_gx1
    }
    #[doc = "0xb4 - point of X-axis of g channel gamma curve register 2"]
    #[inline(always)]
    pub const fn gamma_gx2(&self) -> &GammaGx2 {
        &self.gamma_gx2
    }
    #[doc = "0xb8 - point of X-axis of b channel gamma curve register 1"]
    #[inline(always)]
    pub const fn gamma_bx1(&self) -> &GammaBx1 {
        &self.gamma_bx1
    }
    #[doc = "0xbc - point of X-axis of b channel gamma curve register 2"]
    #[inline(always)]
    pub const fn gamma_bx2(&self) -> &GammaBx2 {
        &self.gamma_bx2
    }
    #[doc = "0xc0 - ae control register"]
    #[inline(always)]
    pub const fn ae_ctrl(&self) -> &AeCtrl {
        &self.ae_ctrl
    }
    #[doc = "0xc4 - ae monitor control register"]
    #[inline(always)]
    pub const fn ae_monitor(&self) -> &AeMonitor {
        &self.ae_monitor
    }
    #[doc = "0xc8 - ae window register in x-direction"]
    #[inline(always)]
    pub const fn ae_bx(&self) -> &AeBx {
        &self.ae_bx
    }
    #[doc = "0xcc - ae window register in y-direction"]
    #[inline(always)]
    pub const fn ae_by(&self) -> &AeBy {
        &self.ae_by
    }
    #[doc = "0xd0 - ae sub-window pix num register"]
    #[inline(always)]
    pub const fn ae_winpixnum(&self) -> &AeWinpixnum {
        &self.ae_winpixnum
    }
    #[doc = "0xd4 - reciprocal of ae sub-window pixel number"]
    #[inline(always)]
    pub const fn ae_win_reciprocal(&self) -> &AeWinReciprocal {
        &self.ae_win_reciprocal
    }
    #[doc = "0xd8 - ae statistic result register 0"]
    #[inline(always)]
    pub const fn ae_block_mean_0(&self) -> &AeBlockMean0 {
        &self.ae_block_mean_0
    }
    #[doc = "0xdc - ae statistic result register 1"]
    #[inline(always)]
    pub const fn ae_block_mean_1(&self) -> &AeBlockMean1 {
        &self.ae_block_mean_1
    }
    #[doc = "0xe0 - ae statistic result register 2"]
    #[inline(always)]
    pub const fn ae_block_mean_2(&self) -> &AeBlockMean2 {
        &self.ae_block_mean_2
    }
    #[doc = "0xe4 - ae statistic result register 3"]
    #[inline(always)]
    pub const fn ae_block_mean_3(&self) -> &AeBlockMean3 {
        &self.ae_block_mean_3
    }
    #[doc = "0xe8 - ae statistic result register 4"]
    #[inline(always)]
    pub const fn ae_block_mean_4(&self) -> &AeBlockMean4 {
        &self.ae_block_mean_4
    }
    #[doc = "0xec - ae statistic result register 5"]
    #[inline(always)]
    pub const fn ae_block_mean_5(&self) -> &AeBlockMean5 {
        &self.ae_block_mean_5
    }
    #[doc = "0xf0 - ae statistic result register 6"]
    #[inline(always)]
    pub const fn ae_block_mean_6(&self) -> &AeBlockMean6 {
        &self.ae_block_mean_6
    }
    #[doc = "0xf4 - sharp control register 0"]
    #[inline(always)]
    pub const fn sharp_ctrl0(&self) -> &SharpCtrl0 {
        &self.sharp_ctrl0
    }
    #[doc = "0xf8 - sharp usm config register 0"]
    #[inline(always)]
    pub const fn sharp_filter0(&self) -> &SharpFilter0 {
        &self.sharp_filter0
    }
    #[doc = "0xfc - sharp usm config register 1"]
    #[inline(always)]
    pub const fn sharp_filter1(&self) -> &SharpFilter1 {
        &self.sharp_filter1
    }
    #[doc = "0x100 - sharp usm config register 2"]
    #[inline(always)]
    pub const fn sharp_filter2(&self) -> &SharpFilter2 {
        &self.sharp_filter2
    }
    #[doc = "0x104 - sharp pix2matrix ctrl"]
    #[inline(always)]
    pub const fn sharp_matrix_ctrl(&self) -> &SharpMatrixCtrl {
        &self.sharp_matrix_ctrl
    }
    #[doc = "0x108 - sharp control register 1"]
    #[inline(always)]
    pub const fn sharp_ctrl1(&self) -> &SharpCtrl1 {
        &self.sharp_ctrl1
    }
    #[doc = "0x10c - isp dma source trans control register"]
    #[inline(always)]
    pub const fn dma_cntl(&self) -> &DmaCntl {
        &self.dma_cntl
    }
    #[doc = "0x110 - isp dma source total raw number set register"]
    #[inline(always)]
    pub const fn dma_raw_data(&self) -> &DmaRawData {
        &self.dma_raw_data
    }
    #[doc = "0x114 - isp cam source control register"]
    #[inline(always)]
    pub const fn cam_cntl(&self) -> &CamCntl {
        &self.cam_cntl
    }
    #[doc = "0x118 - isp cam source config register"]
    #[inline(always)]
    pub const fn cam_conf(&self) -> &CamConf {
        &self.cam_conf
    }
    #[doc = "0x11c - af control register 0"]
    #[inline(always)]
    pub const fn af_ctrl0(&self) -> &AfCtrl0 {
        &self.af_ctrl0
    }
    #[doc = "0x120 - af control register 1"]
    #[inline(always)]
    pub const fn af_ctrl1(&self) -> &AfCtrl1 {
        &self.af_ctrl1
    }
    #[doc = "0x124 - af gen threshold control register"]
    #[inline(always)]
    pub const fn af_gen_th_ctrl(&self) -> &AfGenThCtrl {
        &self.af_gen_th_ctrl
    }
    #[doc = "0x128 - af monitor user sum threshold register"]
    #[inline(always)]
    pub const fn af_env_user_th_sum(&self) -> &AfEnvUserThSum {
        &self.af_env_user_th_sum
    }
    #[doc = "0x12c - af monitor user lum threshold register"]
    #[inline(always)]
    pub const fn af_env_user_th_lum(&self) -> &AfEnvUserThLum {
        &self.af_env_user_th_lum
    }
    #[doc = "0x130 - af threshold register"]
    #[inline(always)]
    pub const fn af_threshold(&self) -> &AfThreshold {
        &self.af_threshold
    }
    #[doc = "0x134 - h-scale of af window a register"]
    #[inline(always)]
    pub const fn af_hscale_a(&self) -> &AfHscaleA {
        &self.af_hscale_a
    }
    #[doc = "0x138 - v-scale of af window a register"]
    #[inline(always)]
    pub const fn af_vscale_a(&self) -> &AfVscaleA {
        &self.af_vscale_a
    }
    #[doc = "0x13c - h-scale of af window b register"]
    #[inline(always)]
    pub const fn af_hscale_b(&self) -> &AfHscaleB {
        &self.af_hscale_b
    }
    #[doc = "0x140 - v-scale of af window b register"]
    #[inline(always)]
    pub const fn af_vscale_b(&self) -> &AfVscaleB {
        &self.af_vscale_b
    }
    #[doc = "0x144 - v-scale of af window c register"]
    #[inline(always)]
    pub const fn af_hscale_c(&self) -> &AfHscaleC {
        &self.af_hscale_c
    }
    #[doc = "0x148 - v-scale of af window c register"]
    #[inline(always)]
    pub const fn af_vscale_c(&self) -> &AfVscaleC {
        &self.af_vscale_c
    }
    #[doc = "0x14c - result of sum of af window a"]
    #[inline(always)]
    pub const fn af_sum_a(&self) -> &AfSumA {
        &self.af_sum_a
    }
    #[doc = "0x150 - result of sum of af window b"]
    #[inline(always)]
    pub const fn af_sum_b(&self) -> &AfSumB {
        &self.af_sum_b
    }
    #[doc = "0x154 - result of sum of af window c"]
    #[inline(always)]
    pub const fn af_sum_c(&self) -> &AfSumC {
        &self.af_sum_c
    }
    #[doc = "0x158 - result of lum of af window a"]
    #[inline(always)]
    pub const fn af_lum_a(&self) -> &AfLumA {
        &self.af_lum_a
    }
    #[doc = "0x15c - result of lum of af window b"]
    #[inline(always)]
    pub const fn af_lum_b(&self) -> &AfLumB {
        &self.af_lum_b
    }
    #[doc = "0x160 - result of lum of af window c"]
    #[inline(always)]
    pub const fn af_lum_c(&self) -> &AfLumC {
        &self.af_lum_c
    }
    #[doc = "0x164 - awb mode control register"]
    #[inline(always)]
    pub const fn awb_mode(&self) -> &AwbMode {
        &self.awb_mode
    }
    #[doc = "0x168 - h-scale of awb window"]
    #[inline(always)]
    pub const fn awb_hscale(&self) -> &AwbHscale {
        &self.awb_hscale
    }
    #[doc = "0x16c - v-scale of awb window"]
    #[inline(always)]
    pub const fn awb_vscale(&self) -> &AwbVscale {
        &self.awb_vscale
    }
    #[doc = "0x170 - awb lum threshold register"]
    #[inline(always)]
    pub const fn awb_th_lum(&self) -> &AwbThLum {
        &self.awb_th_lum
    }
    #[doc = "0x174 - awb r/g threshold register"]
    #[inline(always)]
    pub const fn awb_th_rg(&self) -> &AwbThRg {
        &self.awb_th_rg
    }
    #[doc = "0x178 - awb b/g threshold register"]
    #[inline(always)]
    pub const fn awb_th_bg(&self) -> &AwbThBg {
        &self.awb_th_bg
    }
    #[doc = "0x17c - result of awb white point number"]
    #[inline(always)]
    pub const fn awb0_white_cnt(&self) -> &Awb0WhiteCnt {
        &self.awb0_white_cnt
    }
    #[doc = "0x180 - result of accumulate of r channel of all white points"]
    #[inline(always)]
    pub const fn awb0_acc_r(&self) -> &Awb0AccR {
        &self.awb0_acc_r
    }
    #[doc = "0x184 - result of accumulate of g channel of all white points"]
    #[inline(always)]
    pub const fn awb0_acc_g(&self) -> &Awb0AccG {
        &self.awb0_acc_g
    }
    #[doc = "0x188 - result of accumulate of b channel of all white points"]
    #[inline(always)]
    pub const fn awb0_acc_b(&self) -> &Awb0AccB {
        &self.awb0_acc_b
    }
    #[doc = "0x18c - color control register"]
    #[inline(always)]
    pub const fn color_ctrl(&self) -> &ColorCtrl {
        &self.color_ctrl
    }
    #[doc = "0x190 - blc black level register"]
    #[inline(always)]
    pub const fn blc_value(&self) -> &BlcValue {
        &self.blc_value
    }
    #[doc = "0x194 - blc stretch control register"]
    #[inline(always)]
    pub const fn blc_ctrl0(&self) -> &BlcCtrl0 {
        &self.blc_ctrl0
    }
    #[doc = "0x198 - blc window control register"]
    #[inline(always)]
    pub const fn blc_ctrl1(&self) -> &BlcCtrl1 {
        &self.blc_ctrl1
    }
    #[doc = "0x19c - blc black threshold control register"]
    #[inline(always)]
    pub const fn blc_ctrl2(&self) -> &BlcCtrl2 {
        &self.blc_ctrl2
    }
    #[doc = "0x1a0 - results of the average of black window"]
    #[inline(always)]
    pub const fn blc_mean(&self) -> &BlcMean {
        &self.blc_mean
    }
    #[doc = "0x1a4 - histogram mode control register"]
    #[inline(always)]
    pub const fn hist_mode(&self) -> &HistMode {
        &self.hist_mode
    }
    #[doc = "0x1a8 - histogram rgb to gray coefficients register"]
    #[inline(always)]
    pub const fn hist_coeff(&self) -> &HistCoeff {
        &self.hist_coeff
    }
    #[doc = "0x1ac - histogram window offsets register"]
    #[inline(always)]
    pub const fn hist_offs(&self) -> &HistOffs {
        &self.hist_offs
    }
    #[doc = "0x1b0 - histogram sub-window size register"]
    #[inline(always)]
    pub const fn hist_size(&self) -> &HistSize {
        &self.hist_size
    }
    #[doc = "0x1b4 - histogram bin control register 0"]
    #[inline(always)]
    pub const fn hist_seg0(&self) -> &HistSeg0 {
        &self.hist_seg0
    }
    #[doc = "0x1b8 - histogram bin control register 1"]
    #[inline(always)]
    pub const fn hist_seg1(&self) -> &HistSeg1 {
        &self.hist_seg1
    }
    #[doc = "0x1bc - histogram bin control register 2"]
    #[inline(always)]
    pub const fn hist_seg2(&self) -> &HistSeg2 {
        &self.hist_seg2
    }
    #[doc = "0x1c0 - histogram bin control register 3"]
    #[inline(always)]
    pub const fn hist_seg3(&self) -> &HistSeg3 {
        &self.hist_seg3
    }
    #[doc = "0x1c4 - histogram sub-window weight register 0"]
    #[inline(always)]
    pub const fn hist_weight0(&self) -> &HistWeight0 {
        &self.hist_weight0
    }
    #[doc = "0x1c8 - histogram sub-window weight register 1"]
    #[inline(always)]
    pub const fn hist_weight1(&self) -> &HistWeight1 {
        &self.hist_weight1
    }
    #[doc = "0x1cc - histogram sub-window weight register 2"]
    #[inline(always)]
    pub const fn hist_weight2(&self) -> &HistWeight2 {
        &self.hist_weight2
    }
    #[doc = "0x1d0 - histogram sub-window weight register 3"]
    #[inline(always)]
    pub const fn hist_weight3(&self) -> &HistWeight3 {
        &self.hist_weight3
    }
    #[doc = "0x1d4 - histogram sub-window weight register 4"]
    #[inline(always)]
    pub const fn hist_weight4(&self) -> &HistWeight4 {
        &self.hist_weight4
    }
    #[doc = "0x1d8 - histogram sub-window weight register 5"]
    #[inline(always)]
    pub const fn hist_weight5(&self) -> &HistWeight5 {
        &self.hist_weight5
    }
    #[doc = "0x1dc - histogram sub-window weight register 6"]
    #[inline(always)]
    pub const fn hist_weight6(&self) -> &HistWeight6 {
        &self.hist_weight6
    }
    #[doc = "0x1e0 - result of histogram bin 0"]
    #[inline(always)]
    pub const fn hist_bin0(&self) -> &HistBin0 {
        &self.hist_bin0
    }
    #[doc = "0x1e4 - result of histogram bin 1"]
    #[inline(always)]
    pub const fn hist_bin1(&self) -> &HistBin1 {
        &self.hist_bin1
    }
    #[doc = "0x1e8 - result of histogram bin 2"]
    #[inline(always)]
    pub const fn hist_bin2(&self) -> &HistBin2 {
        &self.hist_bin2
    }
    #[doc = "0x1ec - result of histogram bin 3"]
    #[inline(always)]
    pub const fn hist_bin3(&self) -> &HistBin3 {
        &self.hist_bin3
    }
    #[doc = "0x1f0 - result of histogram bin 4"]
    #[inline(always)]
    pub const fn hist_bin4(&self) -> &HistBin4 {
        &self.hist_bin4
    }
    #[doc = "0x1f4 - result of histogram bin 5"]
    #[inline(always)]
    pub const fn hist_bin5(&self) -> &HistBin5 {
        &self.hist_bin5
    }
    #[doc = "0x1f8 - result of histogram bin 6"]
    #[inline(always)]
    pub const fn hist_bin6(&self) -> &HistBin6 {
        &self.hist_bin6
    }
    #[doc = "0x1fc - result of histogram bin 7"]
    #[inline(always)]
    pub const fn hist_bin7(&self) -> &HistBin7 {
        &self.hist_bin7
    }
    #[doc = "0x200 - result of histogram bin 8"]
    #[inline(always)]
    pub const fn hist_bin8(&self) -> &HistBin8 {
        &self.hist_bin8
    }
    #[doc = "0x204 - result of histogram bin 9"]
    #[inline(always)]
    pub const fn hist_bin9(&self) -> &HistBin9 {
        &self.hist_bin9
    }
    #[doc = "0x208 - result of histogram bin 10"]
    #[inline(always)]
    pub const fn hist_bin10(&self) -> &HistBin10 {
        &self.hist_bin10
    }
    #[doc = "0x20c - result of histogram bin 11"]
    #[inline(always)]
    pub const fn hist_bin11(&self) -> &HistBin11 {
        &self.hist_bin11
    }
    #[doc = "0x210 - result of histogram bin 12"]
    #[inline(always)]
    pub const fn hist_bin12(&self) -> &HistBin12 {
        &self.hist_bin12
    }
    #[doc = "0x214 - result of histogram bin 13"]
    #[inline(always)]
    pub const fn hist_bin13(&self) -> &HistBin13 {
        &self.hist_bin13
    }
    #[doc = "0x218 - result of histogram bin 14"]
    #[inline(always)]
    pub const fn hist_bin14(&self) -> &HistBin14 {
        &self.hist_bin14
    }
    #[doc = "0x21c - result of histogram bin 15"]
    #[inline(always)]
    pub const fn hist_bin15(&self) -> &HistBin15 {
        &self.hist_bin15
    }
    #[doc = "0x220 - mem aux control register 0"]
    #[inline(always)]
    pub const fn mem_aux_ctrl_0(&self) -> &MemAuxCtrl0 {
        &self.mem_aux_ctrl_0
    }
    #[doc = "0x224 - mem aux control register 1"]
    #[inline(always)]
    pub const fn mem_aux_ctrl_1(&self) -> &MemAuxCtrl1 {
        &self.mem_aux_ctrl_1
    }
    #[doc = "0x228 - mem aux control register 2"]
    #[inline(always)]
    pub const fn mem_aux_ctrl_2(&self) -> &MemAuxCtrl2 {
        &self.mem_aux_ctrl_2
    }
    #[doc = "0x22c - mem aux control register 3"]
    #[inline(always)]
    pub const fn mem_aux_ctrl_3(&self) -> &MemAuxCtrl3 {
        &self.mem_aux_ctrl_3
    }
    #[doc = "0x230 - mem aux control register 4"]
    #[inline(always)]
    pub const fn mem_aux_ctrl_4(&self) -> &MemAuxCtrl4 {
        &self.mem_aux_ctrl_4
    }
    #[doc = "0x234 - yuv format control register"]
    #[inline(always)]
    pub const fn yuv_format(&self) -> &YuvFormat {
        &self.yuv_format
    }
    #[doc = "0x238 - rdn eco cs register"]
    #[inline(always)]
    pub const fn rdn_eco_cs(&self) -> &RdnEcoCs {
        &self.rdn_eco_cs
    }
    #[doc = "0x23c - rdn eco all low register"]
    #[inline(always)]
    pub const fn rdn_eco_low(&self) -> &RdnEcoLow {
        &self.rdn_eco_low
    }
    #[doc = "0x240 - rdn eco all high register"]
    #[inline(always)]
    pub const fn rdn_eco_high(&self) -> &RdnEcoHigh {
        &self.rdn_eco_high
    }
    #[doc = "0x244 - isp_crop ctrl register"]
    #[inline(always)]
    pub const fn crop_ctrl(&self) -> &CropCtrl {
        &self.crop_ctrl
    }
    #[doc = "0x248 - isp_crop row capture range register"]
    #[inline(always)]
    pub const fn crop_y_capture(&self) -> &CropYCapture {
        &self.crop_y_capture
    }
    #[doc = "0x24c - isp_crop col capture range register"]
    #[inline(always)]
    pub const fn crop_x_capture(&self) -> &CropXCapture {
        &self.crop_x_capture
    }
    #[doc = "0x250 - crop error state register"]
    #[inline(always)]
    pub const fn crop_err_st(&self) -> &CropErrSt {
        &self.crop_err_st
    }
    #[doc = "0x254 - white balance red gain register 0"]
    #[inline(always)]
    pub const fn wbg_coef_r(&self) -> &WbgCoefR {
        &self.wbg_coef_r
    }
    #[doc = "0x258 - white balance green gain register 0"]
    #[inline(always)]
    pub const fn wbg_coef_g(&self) -> &WbgCoefG {
        &self.wbg_coef_g
    }
    #[doc = "0x25c - white balance blue gain register 0"]
    #[inline(always)]
    pub const fn wbg_coef_b(&self) -> &WbgCoefB {
        &self.wbg_coef_b
    }
    #[doc = "0x260 - color control register"]
    #[inline(always)]
    pub const fn color_hue_ctrl(&self) -> &ColorHueCtrl {
        &self.color_hue_ctrl
    }
    #[doc = "0x264 - awb window register in x-direction"]
    #[inline(always)]
    pub const fn awb_bx(&self) -> &AwbBx {
        &self.awb_bx
    }
    #[doc = "0x268 - awb window register in y-direction"]
    #[inline(always)]
    pub const fn awb_by(&self) -> &AwbBy {
        &self.awb_by
    }
    #[doc = "0x26c - awb window register in y-direction"]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
    #[doc = "0x270 - shadow register ctrl register"]
    #[inline(always)]
    pub const fn shadow_reg_ctrl(&self) -> &ShadowRegCtrl {
        &self.shadow_reg_ctrl
    }
}
#[doc = "VER_DATE (rw) register accessor: version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ver_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ver_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ver_date`] module"]
#[doc(alias = "VER_DATE")]
pub type VerDate = crate::Reg<ver_date::VerDateSpec>;
#[doc = "version control register"]
pub mod ver_date;
#[doc = "CLK_EN (rw) register accessor: isp clk control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en`] module"]
#[doc(alias = "CLK_EN")]
pub type ClkEn = crate::Reg<clk_en::ClkEnSpec>;
#[doc = "isp clk control register"]
pub mod clk_en;
#[doc = "CNTL (rw) register accessor: isp module enable control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl`] module"]
#[doc(alias = "CNTL")]
pub type Cntl = crate::Reg<cntl::CntlSpec>;
#[doc = "isp module enable control register"]
pub mod cntl;
#[doc = "HSYNC_CNT (rw) register accessor: header hsync interval control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsync_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsync_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsync_cnt`] module"]
#[doc(alias = "HSYNC_CNT")]
pub type HsyncCnt = crate::Reg<hsync_cnt::HsyncCntSpec>;
#[doc = "header hsync interval control register"]
pub mod hsync_cnt;
#[doc = "FRAME_CFG (rw) register accessor: frame control parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame_cfg`] module"]
#[doc(alias = "FRAME_CFG")]
pub type FrameCfg = crate::Reg<frame_cfg::FrameCfgSpec>;
#[doc = "frame control parameter register"]
pub mod frame_cfg;
#[doc = "CCM_COEF0 (rw) register accessor: ccm coef register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ccm_coef0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccm_coef0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccm_coef0`] module"]
#[doc(alias = "CCM_COEF0")]
pub type CcmCoef0 = crate::Reg<ccm_coef0::CcmCoef0Spec>;
#[doc = "ccm coef register 0"]
pub mod ccm_coef0;
#[doc = "CCM_COEF1 (rw) register accessor: ccm coef register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccm_coef1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccm_coef1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccm_coef1`] module"]
#[doc(alias = "CCM_COEF1")]
pub type CcmCoef1 = crate::Reg<ccm_coef1::CcmCoef1Spec>;
#[doc = "ccm coef register 1"]
pub mod ccm_coef1;
#[doc = "CCM_COEF3 (rw) register accessor: ccm coef register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccm_coef3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccm_coef3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccm_coef3`] module"]
#[doc(alias = "CCM_COEF3")]
pub type CcmCoef3 = crate::Reg<ccm_coef3::CcmCoef3Spec>;
#[doc = "ccm coef register 3"]
pub mod ccm_coef3;
#[doc = "CCM_COEF4 (rw) register accessor: ccm coef register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ccm_coef4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccm_coef4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccm_coef4`] module"]
#[doc(alias = "CCM_COEF4")]
pub type CcmCoef4 = crate::Reg<ccm_coef4::CcmCoef4Spec>;
#[doc = "ccm coef register 4"]
pub mod ccm_coef4;
#[doc = "CCM_COEF5 (rw) register accessor: ccm coef register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`ccm_coef5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccm_coef5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccm_coef5`] module"]
#[doc(alias = "CCM_COEF5")]
pub type CcmCoef5 = crate::Reg<ccm_coef5::CcmCoef5Spec>;
#[doc = "ccm coef register 5"]
pub mod ccm_coef5;
#[doc = "BF_MATRIX_CTRL (rw) register accessor: bf pix2matrix ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`bf_matrix_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bf_matrix_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bf_matrix_ctrl`] module"]
#[doc(alias = "BF_MATRIX_CTRL")]
pub type BfMatrixCtrl = crate::Reg<bf_matrix_ctrl::BfMatrixCtrlSpec>;
#[doc = "bf pix2matrix ctrl"]
pub mod bf_matrix_ctrl;
#[doc = "BF_SIGMA (rw) register accessor: bf denoising level control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bf_sigma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bf_sigma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bf_sigma`] module"]
#[doc(alias = "BF_SIGMA")]
pub type BfSigma = crate::Reg<bf_sigma::BfSigmaSpec>;
#[doc = "bf denoising level control register"]
pub mod bf_sigma;
#[doc = "BF_GAU0 (rw) register accessor: bf gau template register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`bf_gau0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bf_gau0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bf_gau0`] module"]
#[doc(alias = "BF_GAU0")]
pub type BfGau0 = crate::Reg<bf_gau0::BfGau0Spec>;
#[doc = "bf gau template register 0"]
pub mod bf_gau0;
#[doc = "BF_GAU1 (rw) register accessor: bf gau template register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bf_gau1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bf_gau1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bf_gau1`] module"]
#[doc(alias = "BF_GAU1")]
pub type BfGau1 = crate::Reg<bf_gau1::BfGau1Spec>;
#[doc = "bf gau template register 1"]
pub mod bf_gau1;
#[doc = "DPC_CTRL (rw) register accessor: DPC mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dpc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpc_ctrl`] module"]
#[doc(alias = "DPC_CTRL")]
pub type DpcCtrl = crate::Reg<dpc_ctrl::DpcCtrlSpec>;
#[doc = "DPC mode control register"]
pub mod dpc_ctrl;
#[doc = "DPC_CONF (rw) register accessor: DPC parameter config register\n\nYou can [`read`](crate::Reg::read) this register and get [`dpc_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpc_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpc_conf`] module"]
#[doc(alias = "DPC_CONF")]
pub type DpcConf = crate::Reg<dpc_conf::DpcConfSpec>;
#[doc = "DPC parameter config register"]
pub mod dpc_conf;
#[doc = "DPC_MATRIX_CTRL (rw) register accessor: dpc pix2matrix ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`dpc_matrix_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpc_matrix_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpc_matrix_ctrl`] module"]
#[doc(alias = "DPC_MATRIX_CTRL")]
pub type DpcMatrixCtrl = crate::Reg<dpc_matrix_ctrl::DpcMatrixCtrlSpec>;
#[doc = "dpc pix2matrix ctrl"]
pub mod dpc_matrix_ctrl;
#[doc = "DPC_DEADPIX_CNT (r) register accessor: DPC dead-pix number register\n\nYou can [`read`](crate::Reg::read) this register and get [`dpc_deadpix_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpc_deadpix_cnt`] module"]
#[doc(alias = "DPC_DEADPIX_CNT")]
pub type DpcDeadpixCnt = crate::Reg<dpc_deadpix_cnt::DpcDeadpixCntSpec>;
#[doc = "DPC dead-pix number register"]
pub mod dpc_deadpix_cnt;
#[doc = "LUT_CMD (w) register accessor: LUT command register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_cmd`] module"]
#[doc(alias = "LUT_CMD")]
pub type LutCmd = crate::Reg<lut_cmd::LutCmdSpec>;
#[doc = "LUT command register"]
pub mod lut_cmd;
#[doc = "LUT_WDATA (rw) register accessor: LUT write data register\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_wdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_wdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_wdata`] module"]
#[doc(alias = "LUT_WDATA")]
pub type LutWdata = crate::Reg<lut_wdata::LutWdataSpec>;
#[doc = "LUT write data register"]
pub mod lut_wdata;
#[doc = "LUT_RDATA (r) register accessor: LUT read data register\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_rdata`] module"]
#[doc(alias = "LUT_RDATA")]
pub type LutRdata = crate::Reg<lut_rdata::LutRdataSpec>;
#[doc = "LUT read data register"]
pub mod lut_rdata;
#[doc = "LSC_TABLESIZE (rw) register accessor: LSC point in x-direction\n\nYou can [`read`](crate::Reg::read) this register and get [`lsc_tablesize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsc_tablesize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_tablesize`] module"]
#[doc(alias = "LSC_TABLESIZE")]
pub type LscTablesize = crate::Reg<lsc_tablesize::LscTablesizeSpec>;
#[doc = "LSC point in x-direction"]
pub mod lsc_tablesize;
#[doc = "DEMOSAIC_MATRIX_CTRL (rw) register accessor: demosaic pix2matrix ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`demosaic_matrix_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`demosaic_matrix_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@demosaic_matrix_ctrl`] module"]
#[doc(alias = "DEMOSAIC_MATRIX_CTRL")]
pub type DemosaicMatrixCtrl = crate::Reg<demosaic_matrix_ctrl::DemosaicMatrixCtrlSpec>;
#[doc = "demosaic pix2matrix ctrl"]
pub mod demosaic_matrix_ctrl;
#[doc = "DEMOSAIC_GRAD_RATIO (rw) register accessor: demosaic gradient select ratio\n\nYou can [`read`](crate::Reg::read) this register and get [`demosaic_grad_ratio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`demosaic_grad_ratio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@demosaic_grad_ratio`] module"]
#[doc(alias = "DEMOSAIC_GRAD_RATIO")]
pub type DemosaicGradRatio = crate::Reg<demosaic_grad_ratio::DemosaicGradRatioSpec>;
#[doc = "demosaic gradient select ratio"]
pub mod demosaic_grad_ratio;
#[doc = "MEDIAN_MATRIX_CTRL (rw) register accessor: median pix2matrix ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`median_matrix_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`median_matrix_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@median_matrix_ctrl`] module"]
#[doc(alias = "MEDIAN_MATRIX_CTRL")]
pub type MedianMatrixCtrl = crate::Reg<median_matrix_ctrl::MedianMatrixCtrlSpec>;
#[doc = "median pix2matrix ctrl"]
pub mod median_matrix_ctrl;
#[doc = "INT_RAW (r) register accessor: raw interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
#[doc(alias = "INT_RAW")]
pub type IntRaw = crate::Reg<int_raw::IntRawSpec>;
#[doc = "raw interrupt register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: masked interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::IntStSpec>;
#[doc = "masked interrupt register"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
#[doc(alias = "INT_ENA")]
pub type IntEna = crate::Reg<int_ena::IntEnaSpec>;
#[doc = "interrupt enable register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
#[doc = "interrupt clear register"]
pub mod int_clr;
#[doc = "GAMMA_CTRL (rw) register accessor: gamma control register\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_ctrl`] module"]
#[doc(alias = "GAMMA_CTRL")]
pub type GammaCtrl = crate::Reg<gamma_ctrl::GammaCtrlSpec>;
#[doc = "gamma control register"]
pub mod gamma_ctrl;
#[doc = "GAMMA_RY1 (rw) register accessor: point of Y-axis of r channel gamma curve register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_ry1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_ry1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_ry1`] module"]
#[doc(alias = "GAMMA_RY1")]
pub type GammaRy1 = crate::Reg<gamma_ry1::GammaRy1Spec>;
#[doc = "point of Y-axis of r channel gamma curve register 1"]
pub mod gamma_ry1;
#[doc = "GAMMA_RY2 (rw) register accessor: point of Y-axis of r channel gamma curve register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_ry2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_ry2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_ry2`] module"]
#[doc(alias = "GAMMA_RY2")]
pub type GammaRy2 = crate::Reg<gamma_ry2::GammaRy2Spec>;
#[doc = "point of Y-axis of r channel gamma curve register 2"]
pub mod gamma_ry2;
#[doc = "GAMMA_RY3 (rw) register accessor: point of Y-axis of r channel gamma curve register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_ry3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_ry3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_ry3`] module"]
#[doc(alias = "GAMMA_RY3")]
pub type GammaRy3 = crate::Reg<gamma_ry3::GammaRy3Spec>;
#[doc = "point of Y-axis of r channel gamma curve register 3"]
pub mod gamma_ry3;
#[doc = "GAMMA_RY4 (rw) register accessor: point of Y-axis of r channel gamma curve register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_ry4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_ry4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_ry4`] module"]
#[doc(alias = "GAMMA_RY4")]
pub type GammaRy4 = crate::Reg<gamma_ry4::GammaRy4Spec>;
#[doc = "point of Y-axis of r channel gamma curve register 4"]
pub mod gamma_ry4;
#[doc = "GAMMA_GY1 (rw) register accessor: point of Y-axis of g channel gamma curve register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_gy1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_gy1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_gy1`] module"]
#[doc(alias = "GAMMA_GY1")]
pub type GammaGy1 = crate::Reg<gamma_gy1::GammaGy1Spec>;
#[doc = "point of Y-axis of g channel gamma curve register 1"]
pub mod gamma_gy1;
#[doc = "GAMMA_GY2 (rw) register accessor: point of Y-axis of g channel gamma curve register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_gy2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_gy2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_gy2`] module"]
#[doc(alias = "GAMMA_GY2")]
pub type GammaGy2 = crate::Reg<gamma_gy2::GammaGy2Spec>;
#[doc = "point of Y-axis of g channel gamma curve register 2"]
pub mod gamma_gy2;
#[doc = "GAMMA_GY3 (rw) register accessor: point of Y-axis of g channel gamma curve register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_gy3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_gy3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_gy3`] module"]
#[doc(alias = "GAMMA_GY3")]
pub type GammaGy3 = crate::Reg<gamma_gy3::GammaGy3Spec>;
#[doc = "point of Y-axis of g channel gamma curve register 3"]
pub mod gamma_gy3;
#[doc = "GAMMA_GY4 (rw) register accessor: point of Y-axis of g channel gamma curve register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_gy4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_gy4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_gy4`] module"]
#[doc(alias = "GAMMA_GY4")]
pub type GammaGy4 = crate::Reg<gamma_gy4::GammaGy4Spec>;
#[doc = "point of Y-axis of g channel gamma curve register 4"]
pub mod gamma_gy4;
#[doc = "GAMMA_BY1 (rw) register accessor: point of Y-axis of b channel gamma curve register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_by1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_by1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_by1`] module"]
#[doc(alias = "GAMMA_BY1")]
pub type GammaBy1 = crate::Reg<gamma_by1::GammaBy1Spec>;
#[doc = "point of Y-axis of b channel gamma curve register 1"]
pub mod gamma_by1;
#[doc = "GAMMA_BY2 (rw) register accessor: point of Y-axis of b channel gamma curve register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_by2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_by2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_by2`] module"]
#[doc(alias = "GAMMA_BY2")]
pub type GammaBy2 = crate::Reg<gamma_by2::GammaBy2Spec>;
#[doc = "point of Y-axis of b channel gamma curve register 2"]
pub mod gamma_by2;
#[doc = "GAMMA_BY3 (rw) register accessor: point of Y-axis of b channel gamma curve register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_by3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_by3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_by3`] module"]
#[doc(alias = "GAMMA_BY3")]
pub type GammaBy3 = crate::Reg<gamma_by3::GammaBy3Spec>;
#[doc = "point of Y-axis of b channel gamma curve register 3"]
pub mod gamma_by3;
#[doc = "GAMMA_BY4 (rw) register accessor: point of Y-axis of b channel gamma curve register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_by4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_by4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_by4`] module"]
#[doc(alias = "GAMMA_BY4")]
pub type GammaBy4 = crate::Reg<gamma_by4::GammaBy4Spec>;
#[doc = "point of Y-axis of b channel gamma curve register 4"]
pub mod gamma_by4;
#[doc = "GAMMA_RX1 (rw) register accessor: point of X-axis of r channel gamma curve register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_rx1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_rx1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_rx1`] module"]
#[doc(alias = "GAMMA_RX1")]
pub type GammaRx1 = crate::Reg<gamma_rx1::GammaRx1Spec>;
#[doc = "point of X-axis of r channel gamma curve register 1"]
pub mod gamma_rx1;
#[doc = "GAMMA_RX2 (rw) register accessor: point of X-axis of r channel gamma curve register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_rx2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_rx2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_rx2`] module"]
#[doc(alias = "GAMMA_RX2")]
pub type GammaRx2 = crate::Reg<gamma_rx2::GammaRx2Spec>;
#[doc = "point of X-axis of r channel gamma curve register 2"]
pub mod gamma_rx2;
#[doc = "GAMMA_GX1 (rw) register accessor: point of X-axis of g channel gamma curve register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_gx1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_gx1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_gx1`] module"]
#[doc(alias = "GAMMA_GX1")]
pub type GammaGx1 = crate::Reg<gamma_gx1::GammaGx1Spec>;
#[doc = "point of X-axis of g channel gamma curve register 1"]
pub mod gamma_gx1;
#[doc = "GAMMA_GX2 (rw) register accessor: point of X-axis of g channel gamma curve register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_gx2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_gx2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_gx2`] module"]
#[doc(alias = "GAMMA_GX2")]
pub type GammaGx2 = crate::Reg<gamma_gx2::GammaGx2Spec>;
#[doc = "point of X-axis of g channel gamma curve register 2"]
pub mod gamma_gx2;
#[doc = "GAMMA_BX1 (rw) register accessor: point of X-axis of b channel gamma curve register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_bx1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_bx1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_bx1`] module"]
#[doc(alias = "GAMMA_BX1")]
pub type GammaBx1 = crate::Reg<gamma_bx1::GammaBx1Spec>;
#[doc = "point of X-axis of b channel gamma curve register 1"]
pub mod gamma_bx1;
#[doc = "GAMMA_BX2 (rw) register accessor: point of X-axis of b channel gamma curve register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_bx2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_bx2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_bx2`] module"]
#[doc(alias = "GAMMA_BX2")]
pub type GammaBx2 = crate::Reg<gamma_bx2::GammaBx2Spec>;
#[doc = "point of X-axis of b channel gamma curve register 2"]
pub mod gamma_bx2;
#[doc = "AE_CTRL (rw) register accessor: ae control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ae_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ae_ctrl`] module"]
#[doc(alias = "AE_CTRL")]
pub type AeCtrl = crate::Reg<ae_ctrl::AeCtrlSpec>;
#[doc = "ae control register"]
pub mod ae_ctrl;
#[doc = "AE_MONITOR (rw) register accessor: ae monitor control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_monitor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ae_monitor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ae_monitor`] module"]
#[doc(alias = "AE_MONITOR")]
pub type AeMonitor = crate::Reg<ae_monitor::AeMonitorSpec>;
#[doc = "ae monitor control register"]
pub mod ae_monitor;
#[doc = "AE_BX (rw) register accessor: ae window register in x-direction\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_bx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ae_bx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ae_bx`] module"]
#[doc(alias = "AE_BX")]
pub type AeBx = crate::Reg<ae_bx::AeBxSpec>;
#[doc = "ae window register in x-direction"]
pub mod ae_bx;
#[doc = "AE_BY (rw) register accessor: ae window register in y-direction\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ae_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ae_by`] module"]
#[doc(alias = "AE_BY")]
pub type AeBy = crate::Reg<ae_by::AeBySpec>;
#[doc = "ae window register in y-direction"]
pub mod ae_by;
#[doc = "AE_WINPIXNUM (rw) register accessor: ae sub-window pix num register\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_winpixnum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ae_winpixnum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ae_winpixnum`] module"]
#[doc(alias = "AE_WINPIXNUM")]
pub type AeWinpixnum = crate::Reg<ae_winpixnum::AeWinpixnumSpec>;
#[doc = "ae sub-window pix num register"]
pub mod ae_winpixnum;
#[doc = "AE_WIN_RECIPROCAL (rw) register accessor: reciprocal of ae sub-window pixel number\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_win_reciprocal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ae_win_reciprocal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ae_win_reciprocal`] module"]
#[doc(alias = "AE_WIN_RECIPROCAL")]
pub type AeWinReciprocal = crate::Reg<ae_win_reciprocal::AeWinReciprocalSpec>;
#[doc = "reciprocal of ae sub-window pixel number"]
pub mod ae_win_reciprocal;
#[doc = "AE_BLOCK_MEAN_0 (r) register accessor: ae statistic result register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_block_mean_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ae_block_mean_0`] module"]
#[doc(alias = "AE_BLOCK_MEAN_0")]
pub type AeBlockMean0 = crate::Reg<ae_block_mean_0::AeBlockMean0Spec>;
#[doc = "ae statistic result register 0"]
pub mod ae_block_mean_0;
#[doc = "AE_BLOCK_MEAN_1 (r) register accessor: ae statistic result register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_block_mean_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ae_block_mean_1`] module"]
#[doc(alias = "AE_BLOCK_MEAN_1")]
pub type AeBlockMean1 = crate::Reg<ae_block_mean_1::AeBlockMean1Spec>;
#[doc = "ae statistic result register 1"]
pub mod ae_block_mean_1;
#[doc = "AE_BLOCK_MEAN_2 (r) register accessor: ae statistic result register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_block_mean_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ae_block_mean_2`] module"]
#[doc(alias = "AE_BLOCK_MEAN_2")]
pub type AeBlockMean2 = crate::Reg<ae_block_mean_2::AeBlockMean2Spec>;
#[doc = "ae statistic result register 2"]
pub mod ae_block_mean_2;
#[doc = "AE_BLOCK_MEAN_3 (r) register accessor: ae statistic result register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_block_mean_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ae_block_mean_3`] module"]
#[doc(alias = "AE_BLOCK_MEAN_3")]
pub type AeBlockMean3 = crate::Reg<ae_block_mean_3::AeBlockMean3Spec>;
#[doc = "ae statistic result register 3"]
pub mod ae_block_mean_3;
#[doc = "AE_BLOCK_MEAN_4 (r) register accessor: ae statistic result register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_block_mean_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ae_block_mean_4`] module"]
#[doc(alias = "AE_BLOCK_MEAN_4")]
pub type AeBlockMean4 = crate::Reg<ae_block_mean_4::AeBlockMean4Spec>;
#[doc = "ae statistic result register 4"]
pub mod ae_block_mean_4;
#[doc = "AE_BLOCK_MEAN_5 (r) register accessor: ae statistic result register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_block_mean_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ae_block_mean_5`] module"]
#[doc(alias = "AE_BLOCK_MEAN_5")]
pub type AeBlockMean5 = crate::Reg<ae_block_mean_5::AeBlockMean5Spec>;
#[doc = "ae statistic result register 5"]
pub mod ae_block_mean_5;
#[doc = "AE_BLOCK_MEAN_6 (r) register accessor: ae statistic result register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_block_mean_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ae_block_mean_6`] module"]
#[doc(alias = "AE_BLOCK_MEAN_6")]
pub type AeBlockMean6 = crate::Reg<ae_block_mean_6::AeBlockMean6Spec>;
#[doc = "ae statistic result register 6"]
pub mod ae_block_mean_6;
#[doc = "SHARP_CTRL0 (rw) register accessor: sharp control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sharp_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sharp_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sharp_ctrl0`] module"]
#[doc(alias = "SHARP_CTRL0")]
pub type SharpCtrl0 = crate::Reg<sharp_ctrl0::SharpCtrl0Spec>;
#[doc = "sharp control register 0"]
pub mod sharp_ctrl0;
#[doc = "SHARP_FILTER0 (rw) register accessor: sharp usm config register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sharp_filter0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sharp_filter0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sharp_filter0`] module"]
#[doc(alias = "SHARP_FILTER0")]
pub type SharpFilter0 = crate::Reg<sharp_filter0::SharpFilter0Spec>;
#[doc = "sharp usm config register 0"]
pub mod sharp_filter0;
#[doc = "SHARP_FILTER1 (rw) register accessor: sharp usm config register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sharp_filter1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sharp_filter1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sharp_filter1`] module"]
#[doc(alias = "SHARP_FILTER1")]
pub type SharpFilter1 = crate::Reg<sharp_filter1::SharpFilter1Spec>;
#[doc = "sharp usm config register 1"]
pub mod sharp_filter1;
#[doc = "SHARP_FILTER2 (rw) register accessor: sharp usm config register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sharp_filter2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sharp_filter2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sharp_filter2`] module"]
#[doc(alias = "SHARP_FILTER2")]
pub type SharpFilter2 = crate::Reg<sharp_filter2::SharpFilter2Spec>;
#[doc = "sharp usm config register 2"]
pub mod sharp_filter2;
#[doc = "SHARP_MATRIX_CTRL (rw) register accessor: sharp pix2matrix ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`sharp_matrix_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sharp_matrix_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sharp_matrix_ctrl`] module"]
#[doc(alias = "SHARP_MATRIX_CTRL")]
pub type SharpMatrixCtrl = crate::Reg<sharp_matrix_ctrl::SharpMatrixCtrlSpec>;
#[doc = "sharp pix2matrix ctrl"]
pub mod sharp_matrix_ctrl;
#[doc = "SHARP_CTRL1 (r) register accessor: sharp control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sharp_ctrl1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sharp_ctrl1`] module"]
#[doc(alias = "SHARP_CTRL1")]
pub type SharpCtrl1 = crate::Reg<sharp_ctrl1::SharpCtrl1Spec>;
#[doc = "sharp control register 1"]
pub mod sharp_ctrl1;
#[doc = "DMA_CNTL (rw) register accessor: isp dma source trans control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cntl`] module"]
#[doc(alias = "DMA_CNTL")]
pub type DmaCntl = crate::Reg<dma_cntl::DmaCntlSpec>;
#[doc = "isp dma source trans control register"]
pub mod dma_cntl;
#[doc = "DMA_RAW_DATA (rw) register accessor: isp dma source total raw number set register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_raw_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_raw_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_raw_data`] module"]
#[doc(alias = "DMA_RAW_DATA")]
pub type DmaRawData = crate::Reg<dma_raw_data::DmaRawDataSpec>;
#[doc = "isp dma source total raw number set register"]
pub mod dma_raw_data;
#[doc = "CAM_CNTL (rw) register accessor: isp cam source control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cam_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cam_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cam_cntl`] module"]
#[doc(alias = "CAM_CNTL")]
pub type CamCntl = crate::Reg<cam_cntl::CamCntlSpec>;
#[doc = "isp cam source control register"]
pub mod cam_cntl;
#[doc = "CAM_CONF (rw) register accessor: isp cam source config register\n\nYou can [`read`](crate::Reg::read) this register and get [`cam_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cam_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cam_conf`] module"]
#[doc(alias = "CAM_CONF")]
pub type CamConf = crate::Reg<cam_conf::CamConfSpec>;
#[doc = "isp cam source config register"]
pub mod cam_conf;
#[doc = "AF_CTRL0 (rw) register accessor: af control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`af_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af_ctrl0`] module"]
#[doc(alias = "AF_CTRL0")]
pub type AfCtrl0 = crate::Reg<af_ctrl0::AfCtrl0Spec>;
#[doc = "af control register 0"]
pub mod af_ctrl0;
#[doc = "AF_CTRL1 (rw) register accessor: af control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`af_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af_ctrl1`] module"]
#[doc(alias = "AF_CTRL1")]
pub type AfCtrl1 = crate::Reg<af_ctrl1::AfCtrl1Spec>;
#[doc = "af control register 1"]
pub mod af_ctrl1;
#[doc = "AF_GEN_TH_CTRL (rw) register accessor: af gen threshold control register\n\nYou can [`read`](crate::Reg::read) this register and get [`af_gen_th_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_gen_th_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af_gen_th_ctrl`] module"]
#[doc(alias = "AF_GEN_TH_CTRL")]
pub type AfGenThCtrl = crate::Reg<af_gen_th_ctrl::AfGenThCtrlSpec>;
#[doc = "af gen threshold control register"]
pub mod af_gen_th_ctrl;
#[doc = "AF_ENV_USER_TH_SUM (rw) register accessor: af monitor user sum threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`af_env_user_th_sum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_env_user_th_sum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af_env_user_th_sum`] module"]
#[doc(alias = "AF_ENV_USER_TH_SUM")]
pub type AfEnvUserThSum = crate::Reg<af_env_user_th_sum::AfEnvUserThSumSpec>;
#[doc = "af monitor user sum threshold register"]
pub mod af_env_user_th_sum;
#[doc = "AF_ENV_USER_TH_LUM (rw) register accessor: af monitor user lum threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`af_env_user_th_lum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_env_user_th_lum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af_env_user_th_lum`] module"]
#[doc(alias = "AF_ENV_USER_TH_LUM")]
pub type AfEnvUserThLum = crate::Reg<af_env_user_th_lum::AfEnvUserThLumSpec>;
#[doc = "af monitor user lum threshold register"]
pub mod af_env_user_th_lum;
#[doc = "AF_THRESHOLD (rw) register accessor: af threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`af_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af_threshold`] module"]
#[doc(alias = "AF_THRESHOLD")]
pub type AfThreshold = crate::Reg<af_threshold::AfThresholdSpec>;
#[doc = "af threshold register"]
pub mod af_threshold;
#[doc = "AF_HSCALE_A (rw) register accessor: h-scale of af window a register\n\nYou can [`read`](crate::Reg::read) this register and get [`af_hscale_a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_hscale_a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af_hscale_a`] module"]
#[doc(alias = "AF_HSCALE_A")]
pub type AfHscaleA = crate::Reg<af_hscale_a::AfHscaleASpec>;
#[doc = "h-scale of af window a register"]
pub mod af_hscale_a;
#[doc = "AF_VSCALE_A (rw) register accessor: v-scale of af window a register\n\nYou can [`read`](crate::Reg::read) this register and get [`af_vscale_a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_vscale_a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af_vscale_a`] module"]
#[doc(alias = "AF_VSCALE_A")]
pub type AfVscaleA = crate::Reg<af_vscale_a::AfVscaleASpec>;
#[doc = "v-scale of af window a register"]
pub mod af_vscale_a;
#[doc = "AF_HSCALE_B (rw) register accessor: h-scale of af window b register\n\nYou can [`read`](crate::Reg::read) this register and get [`af_hscale_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_hscale_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af_hscale_b`] module"]
#[doc(alias = "AF_HSCALE_B")]
pub type AfHscaleB = crate::Reg<af_hscale_b::AfHscaleBSpec>;
#[doc = "h-scale of af window b register"]
pub mod af_hscale_b;
#[doc = "AF_VSCALE_B (rw) register accessor: v-scale of af window b register\n\nYou can [`read`](crate::Reg::read) this register and get [`af_vscale_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_vscale_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af_vscale_b`] module"]
#[doc(alias = "AF_VSCALE_B")]
pub type AfVscaleB = crate::Reg<af_vscale_b::AfVscaleBSpec>;
#[doc = "v-scale of af window b register"]
pub mod af_vscale_b;
#[doc = "AF_HSCALE_C (rw) register accessor: v-scale of af window c register\n\nYou can [`read`](crate::Reg::read) this register and get [`af_hscale_c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_hscale_c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af_hscale_c`] module"]
#[doc(alias = "AF_HSCALE_C")]
pub type AfHscaleC = crate::Reg<af_hscale_c::AfHscaleCSpec>;
#[doc = "v-scale of af window c register"]
pub mod af_hscale_c;
#[doc = "AF_VSCALE_C (rw) register accessor: v-scale of af window c register\n\nYou can [`read`](crate::Reg::read) this register and get [`af_vscale_c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_vscale_c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af_vscale_c`] module"]
#[doc(alias = "AF_VSCALE_C")]
pub type AfVscaleC = crate::Reg<af_vscale_c::AfVscaleCSpec>;
#[doc = "v-scale of af window c register"]
pub mod af_vscale_c;
#[doc = "AF_SUM_A (r) register accessor: result of sum of af window a\n\nYou can [`read`](crate::Reg::read) this register and get [`af_sum_a::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af_sum_a`] module"]
#[doc(alias = "AF_SUM_A")]
pub type AfSumA = crate::Reg<af_sum_a::AfSumASpec>;
#[doc = "result of sum of af window a"]
pub mod af_sum_a;
#[doc = "AF_SUM_B (r) register accessor: result of sum of af window b\n\nYou can [`read`](crate::Reg::read) this register and get [`af_sum_b::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af_sum_b`] module"]
#[doc(alias = "AF_SUM_B")]
pub type AfSumB = crate::Reg<af_sum_b::AfSumBSpec>;
#[doc = "result of sum of af window b"]
pub mod af_sum_b;
#[doc = "AF_SUM_C (r) register accessor: result of sum of af window c\n\nYou can [`read`](crate::Reg::read) this register and get [`af_sum_c::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af_sum_c`] module"]
#[doc(alias = "AF_SUM_C")]
pub type AfSumC = crate::Reg<af_sum_c::AfSumCSpec>;
#[doc = "result of sum of af window c"]
pub mod af_sum_c;
#[doc = "AF_LUM_A (r) register accessor: result of lum of af window a\n\nYou can [`read`](crate::Reg::read) this register and get [`af_lum_a::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af_lum_a`] module"]
#[doc(alias = "AF_LUM_A")]
pub type AfLumA = crate::Reg<af_lum_a::AfLumASpec>;
#[doc = "result of lum of af window a"]
pub mod af_lum_a;
#[doc = "AF_LUM_B (r) register accessor: result of lum of af window b\n\nYou can [`read`](crate::Reg::read) this register and get [`af_lum_b::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af_lum_b`] module"]
#[doc(alias = "AF_LUM_B")]
pub type AfLumB = crate::Reg<af_lum_b::AfLumBSpec>;
#[doc = "result of lum of af window b"]
pub mod af_lum_b;
#[doc = "AF_LUM_C (r) register accessor: result of lum of af window c\n\nYou can [`read`](crate::Reg::read) this register and get [`af_lum_c::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af_lum_c`] module"]
#[doc(alias = "AF_LUM_C")]
pub type AfLumC = crate::Reg<af_lum_c::AfLumCSpec>;
#[doc = "result of lum of af window c"]
pub mod af_lum_c;
#[doc = "AWB_MODE (rw) register accessor: awb mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`awb_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awb_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_mode`] module"]
#[doc(alias = "AWB_MODE")]
pub type AwbMode = crate::Reg<awb_mode::AwbModeSpec>;
#[doc = "awb mode control register"]
pub mod awb_mode;
#[doc = "AWB_HSCALE (rw) register accessor: h-scale of awb window\n\nYou can [`read`](crate::Reg::read) this register and get [`awb_hscale::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awb_hscale::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_hscale`] module"]
#[doc(alias = "AWB_HSCALE")]
pub type AwbHscale = crate::Reg<awb_hscale::AwbHscaleSpec>;
#[doc = "h-scale of awb window"]
pub mod awb_hscale;
#[doc = "AWB_VSCALE (rw) register accessor: v-scale of awb window\n\nYou can [`read`](crate::Reg::read) this register and get [`awb_vscale::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awb_vscale::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_vscale`] module"]
#[doc(alias = "AWB_VSCALE")]
pub type AwbVscale = crate::Reg<awb_vscale::AwbVscaleSpec>;
#[doc = "v-scale of awb window"]
pub mod awb_vscale;
#[doc = "AWB_TH_LUM (rw) register accessor: awb lum threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`awb_th_lum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awb_th_lum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_th_lum`] module"]
#[doc(alias = "AWB_TH_LUM")]
pub type AwbThLum = crate::Reg<awb_th_lum::AwbThLumSpec>;
#[doc = "awb lum threshold register"]
pub mod awb_th_lum;
#[doc = "AWB_TH_RG (rw) register accessor: awb r/g threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`awb_th_rg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awb_th_rg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_th_rg`] module"]
#[doc(alias = "AWB_TH_RG")]
pub type AwbThRg = crate::Reg<awb_th_rg::AwbThRgSpec>;
#[doc = "awb r/g threshold register"]
pub mod awb_th_rg;
#[doc = "AWB_TH_BG (rw) register accessor: awb b/g threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`awb_th_bg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awb_th_bg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_th_bg`] module"]
#[doc(alias = "AWB_TH_BG")]
pub type AwbThBg = crate::Reg<awb_th_bg::AwbThBgSpec>;
#[doc = "awb b/g threshold register"]
pub mod awb_th_bg;
#[doc = "AWB0_WHITE_CNT (r) register accessor: result of awb white point number\n\nYou can [`read`](crate::Reg::read) this register and get [`awb0_white_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb0_white_cnt`] module"]
#[doc(alias = "AWB0_WHITE_CNT")]
pub type Awb0WhiteCnt = crate::Reg<awb0_white_cnt::Awb0WhiteCntSpec>;
#[doc = "result of awb white point number"]
pub mod awb0_white_cnt;
#[doc = "AWB0_ACC_R (r) register accessor: result of accumulate of r channel of all white points\n\nYou can [`read`](crate::Reg::read) this register and get [`awb0_acc_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb0_acc_r`] module"]
#[doc(alias = "AWB0_ACC_R")]
pub type Awb0AccR = crate::Reg<awb0_acc_r::Awb0AccRSpec>;
#[doc = "result of accumulate of r channel of all white points"]
pub mod awb0_acc_r;
#[doc = "AWB0_ACC_G (r) register accessor: result of accumulate of g channel of all white points\n\nYou can [`read`](crate::Reg::read) this register and get [`awb0_acc_g::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb0_acc_g`] module"]
#[doc(alias = "AWB0_ACC_G")]
pub type Awb0AccG = crate::Reg<awb0_acc_g::Awb0AccGSpec>;
#[doc = "result of accumulate of g channel of all white points"]
pub mod awb0_acc_g;
#[doc = "AWB0_ACC_B (r) register accessor: result of accumulate of b channel of all white points\n\nYou can [`read`](crate::Reg::read) this register and get [`awb0_acc_b::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb0_acc_b`] module"]
#[doc(alias = "AWB0_ACC_B")]
pub type Awb0AccB = crate::Reg<awb0_acc_b::Awb0AccBSpec>;
#[doc = "result of accumulate of b channel of all white points"]
pub mod awb0_acc_b;
#[doc = "COLOR_CTRL (rw) register accessor: color control register\n\nYou can [`read`](crate::Reg::read) this register and get [`color_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`color_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@color_ctrl`] module"]
#[doc(alias = "COLOR_CTRL")]
pub type ColorCtrl = crate::Reg<color_ctrl::ColorCtrlSpec>;
#[doc = "color control register"]
pub mod color_ctrl;
#[doc = "BLC_VALUE (rw) register accessor: blc black level register\n\nYou can [`read`](crate::Reg::read) this register and get [`blc_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blc_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blc_value`] module"]
#[doc(alias = "BLC_VALUE")]
pub type BlcValue = crate::Reg<blc_value::BlcValueSpec>;
#[doc = "blc black level register"]
pub mod blc_value;
#[doc = "BLC_CTRL0 (rw) register accessor: blc stretch control register\n\nYou can [`read`](crate::Reg::read) this register and get [`blc_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blc_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blc_ctrl0`] module"]
#[doc(alias = "BLC_CTRL0")]
pub type BlcCtrl0 = crate::Reg<blc_ctrl0::BlcCtrl0Spec>;
#[doc = "blc stretch control register"]
pub mod blc_ctrl0;
#[doc = "BLC_CTRL1 (rw) register accessor: blc window control register\n\nYou can [`read`](crate::Reg::read) this register and get [`blc_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blc_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blc_ctrl1`] module"]
#[doc(alias = "BLC_CTRL1")]
pub type BlcCtrl1 = crate::Reg<blc_ctrl1::BlcCtrl1Spec>;
#[doc = "blc window control register"]
pub mod blc_ctrl1;
#[doc = "BLC_CTRL2 (rw) register accessor: blc black threshold control register\n\nYou can [`read`](crate::Reg::read) this register and get [`blc_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blc_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blc_ctrl2`] module"]
#[doc(alias = "BLC_CTRL2")]
pub type BlcCtrl2 = crate::Reg<blc_ctrl2::BlcCtrl2Spec>;
#[doc = "blc black threshold control register"]
pub mod blc_ctrl2;
#[doc = "BLC_MEAN (r) register accessor: results of the average of black window\n\nYou can [`read`](crate::Reg::read) this register and get [`blc_mean::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blc_mean`] module"]
#[doc(alias = "BLC_MEAN")]
pub type BlcMean = crate::Reg<blc_mean::BlcMeanSpec>;
#[doc = "results of the average of black window"]
pub mod blc_mean;
#[doc = "HIST_MODE (rw) register accessor: histogram mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_mode`] module"]
#[doc(alias = "HIST_MODE")]
pub type HistMode = crate::Reg<hist_mode::HistModeSpec>;
#[doc = "histogram mode control register"]
pub mod hist_mode;
#[doc = "HIST_COEFF (rw) register accessor: histogram rgb to gray coefficients register\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_coeff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_coeff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_coeff`] module"]
#[doc(alias = "HIST_COEFF")]
pub type HistCoeff = crate::Reg<hist_coeff::HistCoeffSpec>;
#[doc = "histogram rgb to gray coefficients register"]
pub mod hist_coeff;
#[doc = "HIST_OFFS (rw) register accessor: histogram window offsets register\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_offs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_offs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_offs`] module"]
#[doc(alias = "HIST_OFFS")]
pub type HistOffs = crate::Reg<hist_offs::HistOffsSpec>;
#[doc = "histogram window offsets register"]
pub mod hist_offs;
#[doc = "HIST_SIZE (rw) register accessor: histogram sub-window size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_size`] module"]
#[doc(alias = "HIST_SIZE")]
pub type HistSize = crate::Reg<hist_size::HistSizeSpec>;
#[doc = "histogram sub-window size register"]
pub mod hist_size;
#[doc = "HIST_SEG0 (rw) register accessor: histogram bin control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_seg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_seg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_seg0`] module"]
#[doc(alias = "HIST_SEG0")]
pub type HistSeg0 = crate::Reg<hist_seg0::HistSeg0Spec>;
#[doc = "histogram bin control register 0"]
pub mod hist_seg0;
#[doc = "HIST_SEG1 (rw) register accessor: histogram bin control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_seg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_seg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_seg1`] module"]
#[doc(alias = "HIST_SEG1")]
pub type HistSeg1 = crate::Reg<hist_seg1::HistSeg1Spec>;
#[doc = "histogram bin control register 1"]
pub mod hist_seg1;
#[doc = "HIST_SEG2 (rw) register accessor: histogram bin control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_seg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_seg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_seg2`] module"]
#[doc(alias = "HIST_SEG2")]
pub type HistSeg2 = crate::Reg<hist_seg2::HistSeg2Spec>;
#[doc = "histogram bin control register 2"]
pub mod hist_seg2;
#[doc = "HIST_SEG3 (rw) register accessor: histogram bin control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_seg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_seg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_seg3`] module"]
#[doc(alias = "HIST_SEG3")]
pub type HistSeg3 = crate::Reg<hist_seg3::HistSeg3Spec>;
#[doc = "histogram bin control register 3"]
pub mod hist_seg3;
#[doc = "HIST_WEIGHT0 (rw) register accessor: histogram sub-window weight register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_weight0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_weight0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_weight0`] module"]
#[doc(alias = "HIST_WEIGHT0")]
pub type HistWeight0 = crate::Reg<hist_weight0::HistWeight0Spec>;
#[doc = "histogram sub-window weight register 0"]
pub mod hist_weight0;
#[doc = "HIST_WEIGHT1 (rw) register accessor: histogram sub-window weight register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_weight1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_weight1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_weight1`] module"]
#[doc(alias = "HIST_WEIGHT1")]
pub type HistWeight1 = crate::Reg<hist_weight1::HistWeight1Spec>;
#[doc = "histogram sub-window weight register 1"]
pub mod hist_weight1;
#[doc = "HIST_WEIGHT2 (rw) register accessor: histogram sub-window weight register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_weight2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_weight2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_weight2`] module"]
#[doc(alias = "HIST_WEIGHT2")]
pub type HistWeight2 = crate::Reg<hist_weight2::HistWeight2Spec>;
#[doc = "histogram sub-window weight register 2"]
pub mod hist_weight2;
#[doc = "HIST_WEIGHT3 (rw) register accessor: histogram sub-window weight register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_weight3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_weight3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_weight3`] module"]
#[doc(alias = "HIST_WEIGHT3")]
pub type HistWeight3 = crate::Reg<hist_weight3::HistWeight3Spec>;
#[doc = "histogram sub-window weight register 3"]
pub mod hist_weight3;
#[doc = "HIST_WEIGHT4 (rw) register accessor: histogram sub-window weight register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_weight4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_weight4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_weight4`] module"]
#[doc(alias = "HIST_WEIGHT4")]
pub type HistWeight4 = crate::Reg<hist_weight4::HistWeight4Spec>;
#[doc = "histogram sub-window weight register 4"]
pub mod hist_weight4;
#[doc = "HIST_WEIGHT5 (rw) register accessor: histogram sub-window weight register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_weight5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_weight5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_weight5`] module"]
#[doc(alias = "HIST_WEIGHT5")]
pub type HistWeight5 = crate::Reg<hist_weight5::HistWeight5Spec>;
#[doc = "histogram sub-window weight register 5"]
pub mod hist_weight5;
#[doc = "HIST_WEIGHT6 (rw) register accessor: histogram sub-window weight register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_weight6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_weight6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_weight6`] module"]
#[doc(alias = "HIST_WEIGHT6")]
pub type HistWeight6 = crate::Reg<hist_weight6::HistWeight6Spec>;
#[doc = "histogram sub-window weight register 6"]
pub mod hist_weight6;
#[doc = "HIST_BIN0 (r) register accessor: result of histogram bin 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_bin0`] module"]
#[doc(alias = "HIST_BIN0")]
pub type HistBin0 = crate::Reg<hist_bin0::HistBin0Spec>;
#[doc = "result of histogram bin 0"]
pub mod hist_bin0;
#[doc = "HIST_BIN1 (r) register accessor: result of histogram bin 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_bin1`] module"]
#[doc(alias = "HIST_BIN1")]
pub type HistBin1 = crate::Reg<hist_bin1::HistBin1Spec>;
#[doc = "result of histogram bin 1"]
pub mod hist_bin1;
#[doc = "HIST_BIN2 (r) register accessor: result of histogram bin 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_bin2`] module"]
#[doc(alias = "HIST_BIN2")]
pub type HistBin2 = crate::Reg<hist_bin2::HistBin2Spec>;
#[doc = "result of histogram bin 2"]
pub mod hist_bin2;
#[doc = "HIST_BIN3 (r) register accessor: result of histogram bin 3\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_bin3`] module"]
#[doc(alias = "HIST_BIN3")]
pub type HistBin3 = crate::Reg<hist_bin3::HistBin3Spec>;
#[doc = "result of histogram bin 3"]
pub mod hist_bin3;
#[doc = "HIST_BIN4 (r) register accessor: result of histogram bin 4\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_bin4`] module"]
#[doc(alias = "HIST_BIN4")]
pub type HistBin4 = crate::Reg<hist_bin4::HistBin4Spec>;
#[doc = "result of histogram bin 4"]
pub mod hist_bin4;
#[doc = "HIST_BIN5 (r) register accessor: result of histogram bin 5\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_bin5`] module"]
#[doc(alias = "HIST_BIN5")]
pub type HistBin5 = crate::Reg<hist_bin5::HistBin5Spec>;
#[doc = "result of histogram bin 5"]
pub mod hist_bin5;
#[doc = "HIST_BIN6 (r) register accessor: result of histogram bin 6\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_bin6`] module"]
#[doc(alias = "HIST_BIN6")]
pub type HistBin6 = crate::Reg<hist_bin6::HistBin6Spec>;
#[doc = "result of histogram bin 6"]
pub mod hist_bin6;
#[doc = "HIST_BIN7 (r) register accessor: result of histogram bin 7\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_bin7`] module"]
#[doc(alias = "HIST_BIN7")]
pub type HistBin7 = crate::Reg<hist_bin7::HistBin7Spec>;
#[doc = "result of histogram bin 7"]
pub mod hist_bin7;
#[doc = "HIST_BIN8 (r) register accessor: result of histogram bin 8\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_bin8`] module"]
#[doc(alias = "HIST_BIN8")]
pub type HistBin8 = crate::Reg<hist_bin8::HistBin8Spec>;
#[doc = "result of histogram bin 8"]
pub mod hist_bin8;
#[doc = "HIST_BIN9 (r) register accessor: result of histogram bin 9\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_bin9`] module"]
#[doc(alias = "HIST_BIN9")]
pub type HistBin9 = crate::Reg<hist_bin9::HistBin9Spec>;
#[doc = "result of histogram bin 9"]
pub mod hist_bin9;
#[doc = "HIST_BIN10 (r) register accessor: result of histogram bin 10\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_bin10`] module"]
#[doc(alias = "HIST_BIN10")]
pub type HistBin10 = crate::Reg<hist_bin10::HistBin10Spec>;
#[doc = "result of histogram bin 10"]
pub mod hist_bin10;
#[doc = "HIST_BIN11 (r) register accessor: result of histogram bin 11\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_bin11`] module"]
#[doc(alias = "HIST_BIN11")]
pub type HistBin11 = crate::Reg<hist_bin11::HistBin11Spec>;
#[doc = "result of histogram bin 11"]
pub mod hist_bin11;
#[doc = "HIST_BIN12 (r) register accessor: result of histogram bin 12\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_bin12`] module"]
#[doc(alias = "HIST_BIN12")]
pub type HistBin12 = crate::Reg<hist_bin12::HistBin12Spec>;
#[doc = "result of histogram bin 12"]
pub mod hist_bin12;
#[doc = "HIST_BIN13 (r) register accessor: result of histogram bin 13\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_bin13`] module"]
#[doc(alias = "HIST_BIN13")]
pub type HistBin13 = crate::Reg<hist_bin13::HistBin13Spec>;
#[doc = "result of histogram bin 13"]
pub mod hist_bin13;
#[doc = "HIST_BIN14 (r) register accessor: result of histogram bin 14\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_bin14`] module"]
#[doc(alias = "HIST_BIN14")]
pub type HistBin14 = crate::Reg<hist_bin14::HistBin14Spec>;
#[doc = "result of histogram bin 14"]
pub mod hist_bin14;
#[doc = "HIST_BIN15 (r) register accessor: result of histogram bin 15\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_bin15`] module"]
#[doc(alias = "HIST_BIN15")]
pub type HistBin15 = crate::Reg<hist_bin15::HistBin15Spec>;
#[doc = "result of histogram bin 15"]
pub mod hist_bin15;
#[doc = "MEM_AUX_CTRL_0 (rw) register accessor: mem aux control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_aux_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_aux_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_aux_ctrl_0`] module"]
#[doc(alias = "MEM_AUX_CTRL_0")]
pub type MemAuxCtrl0 = crate::Reg<mem_aux_ctrl_0::MemAuxCtrl0Spec>;
#[doc = "mem aux control register 0"]
pub mod mem_aux_ctrl_0;
#[doc = "MEM_AUX_CTRL_1 (rw) register accessor: mem aux control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_aux_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_aux_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_aux_ctrl_1`] module"]
#[doc(alias = "MEM_AUX_CTRL_1")]
pub type MemAuxCtrl1 = crate::Reg<mem_aux_ctrl_1::MemAuxCtrl1Spec>;
#[doc = "mem aux control register 1"]
pub mod mem_aux_ctrl_1;
#[doc = "MEM_AUX_CTRL_2 (rw) register accessor: mem aux control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_aux_ctrl_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_aux_ctrl_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_aux_ctrl_2`] module"]
#[doc(alias = "MEM_AUX_CTRL_2")]
pub type MemAuxCtrl2 = crate::Reg<mem_aux_ctrl_2::MemAuxCtrl2Spec>;
#[doc = "mem aux control register 2"]
pub mod mem_aux_ctrl_2;
#[doc = "MEM_AUX_CTRL_3 (rw) register accessor: mem aux control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_aux_ctrl_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_aux_ctrl_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_aux_ctrl_3`] module"]
#[doc(alias = "MEM_AUX_CTRL_3")]
pub type MemAuxCtrl3 = crate::Reg<mem_aux_ctrl_3::MemAuxCtrl3Spec>;
#[doc = "mem aux control register 3"]
pub mod mem_aux_ctrl_3;
#[doc = "MEM_AUX_CTRL_4 (rw) register accessor: mem aux control register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_aux_ctrl_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_aux_ctrl_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_aux_ctrl_4`] module"]
#[doc(alias = "MEM_AUX_CTRL_4")]
pub type MemAuxCtrl4 = crate::Reg<mem_aux_ctrl_4::MemAuxCtrl4Spec>;
#[doc = "mem aux control register 4"]
pub mod mem_aux_ctrl_4;
#[doc = "YUV_FORMAT (rw) register accessor: yuv format control register\n\nYou can [`read`](crate::Reg::read) this register and get [`yuv_format::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`yuv_format::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@yuv_format`] module"]
#[doc(alias = "YUV_FORMAT")]
pub type YuvFormat = crate::Reg<yuv_format::YuvFormatSpec>;
#[doc = "yuv format control register"]
pub mod yuv_format;
#[doc = "RDN_ECO_CS (rw) register accessor: rdn eco cs register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdn_eco_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdn_eco_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco_cs`] module"]
#[doc(alias = "RDN_ECO_CS")]
pub type RdnEcoCs = crate::Reg<rdn_eco_cs::RdnEcoCsSpec>;
#[doc = "rdn eco cs register"]
pub mod rdn_eco_cs;
#[doc = "RDN_ECO_LOW (rw) register accessor: rdn eco all low register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdn_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdn_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco_low`] module"]
#[doc(alias = "RDN_ECO_LOW")]
pub type RdnEcoLow = crate::Reg<rdn_eco_low::RdnEcoLowSpec>;
#[doc = "rdn eco all low register"]
pub mod rdn_eco_low;
#[doc = "RDN_ECO_HIGH (rw) register accessor: rdn eco all high register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdn_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdn_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco_high`] module"]
#[doc(alias = "RDN_ECO_HIGH")]
pub type RdnEcoHigh = crate::Reg<rdn_eco_high::RdnEcoHighSpec>;
#[doc = "rdn eco all high register"]
pub mod rdn_eco_high;
#[doc = "CROP_CTRL (w) register accessor: isp_crop ctrl register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crop_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crop_ctrl`] module"]
#[doc(alias = "CROP_CTRL")]
pub type CropCtrl = crate::Reg<crop_ctrl::CropCtrlSpec>;
#[doc = "isp_crop ctrl register"]
pub mod crop_ctrl;
#[doc = "CROP_Y_CAPTURE (rw) register accessor: isp_crop row capture range register\n\nYou can [`read`](crate::Reg::read) this register and get [`crop_y_capture::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crop_y_capture::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crop_y_capture`] module"]
#[doc(alias = "CROP_Y_CAPTURE")]
pub type CropYCapture = crate::Reg<crop_y_capture::CropYCaptureSpec>;
#[doc = "isp_crop row capture range register"]
pub mod crop_y_capture;
#[doc = "CROP_X_CAPTURE (rw) register accessor: isp_crop col capture range register\n\nYou can [`read`](crate::Reg::read) this register and get [`crop_x_capture::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crop_x_capture::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crop_x_capture`] module"]
#[doc(alias = "CROP_X_CAPTURE")]
pub type CropXCapture = crate::Reg<crop_x_capture::CropXCaptureSpec>;
#[doc = "isp_crop col capture range register"]
pub mod crop_x_capture;
#[doc = "CROP_ERR_ST (r) register accessor: crop error state register\n\nYou can [`read`](crate::Reg::read) this register and get [`crop_err_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crop_err_st`] module"]
#[doc(alias = "CROP_ERR_ST")]
pub type CropErrSt = crate::Reg<crop_err_st::CropErrStSpec>;
#[doc = "crop error state register"]
pub mod crop_err_st;
#[doc = "WBG_COEF_R (rw) register accessor: white balance red gain register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wbg_coef_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wbg_coef_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wbg_coef_r`] module"]
#[doc(alias = "WBG_COEF_R")]
pub type WbgCoefR = crate::Reg<wbg_coef_r::WbgCoefRSpec>;
#[doc = "white balance red gain register 0"]
pub mod wbg_coef_r;
#[doc = "WBG_COEF_G (rw) register accessor: white balance green gain register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wbg_coef_g::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wbg_coef_g::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wbg_coef_g`] module"]
#[doc(alias = "WBG_COEF_G")]
pub type WbgCoefG = crate::Reg<wbg_coef_g::WbgCoefGSpec>;
#[doc = "white balance green gain register 0"]
pub mod wbg_coef_g;
#[doc = "WBG_COEF_B (rw) register accessor: white balance blue gain register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wbg_coef_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wbg_coef_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wbg_coef_b`] module"]
#[doc(alias = "WBG_COEF_B")]
pub type WbgCoefB = crate::Reg<wbg_coef_b::WbgCoefBSpec>;
#[doc = "white balance blue gain register 0"]
pub mod wbg_coef_b;
#[doc = "COLOR_HUE_CTRL (rw) register accessor: color control register\n\nYou can [`read`](crate::Reg::read) this register and get [`color_hue_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`color_hue_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@color_hue_ctrl`] module"]
#[doc(alias = "COLOR_HUE_CTRL")]
pub type ColorHueCtrl = crate::Reg<color_hue_ctrl::ColorHueCtrlSpec>;
#[doc = "color control register"]
pub mod color_hue_ctrl;
#[doc = "AWB_BX (rw) register accessor: awb window register in x-direction\n\nYou can [`read`](crate::Reg::read) this register and get [`awb_bx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awb_bx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_bx`] module"]
#[doc(alias = "AWB_BX")]
pub type AwbBx = crate::Reg<awb_bx::AwbBxSpec>;
#[doc = "awb window register in x-direction"]
pub mod awb_bx;
#[doc = "AWB_BY (rw) register accessor: awb window register in y-direction\n\nYou can [`read`](crate::Reg::read) this register and get [`awb_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awb_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_by`] module"]
#[doc(alias = "AWB_BY")]
pub type AwbBy = crate::Reg<awb_by::AwbBySpec>;
#[doc = "awb window register in y-direction"]
pub mod awb_by;
#[doc = "STATE (r) register accessor: awb window register in y-direction\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "awb window register in y-direction"]
pub mod state;
#[doc = "SHADOW_REG_CTRL (rw) register accessor: shadow register ctrl register\n\nYou can [`read`](crate::Reg::read) this register and get [`shadow_reg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shadow_reg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shadow_reg_ctrl`] module"]
#[doc(alias = "SHADOW_REG_CTRL")]
pub type ShadowRegCtrl = crate::Reg<shadow_reg_ctrl::ShadowRegCtrlSpec>;
#[doc = "shadow register ctrl register"]
pub mod shadow_reg_ctrl;
