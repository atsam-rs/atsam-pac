<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <!-- Due to a bug in svd2rust, we need to remove enumeratedValue nodes on fields with bitWidths of 32. -->
  <xsl:template match="/device/peripherals/peripheral/registers/register/fields/field[bitWidth='32']/enumeratedValues">
  </xsl:template>

  <!-- The ADCIFE peripheral has two representations of the CDMA register so we'll rename the second one. -->
  <xsl:param name="cdma_replacement" select="'CDMA_ALT'"/>
  <xsl:template match="/device/peripherals/peripheral[name='ADCIFE']/registers/register[name='CDMA'][2]/name/text()">
    <xsl:value-of select="$cdma_replacement"/>
  </xsl:template>

  <!-- The TC0 peripheral has two representations of the CMR register so we'll rename the second one. -->
  <xsl:param name="cmr_replacement" select="'CMR%s_ALT'"/>
  <xsl:template match="/device/peripherals/peripheral[name='TC0']/registers/register[name='CMR%s'][2]/name/text()">
    <xsl:value-of select="$cmr_replacement"/>
  </xsl:template>

  <!-- The USART0 peripheral has different representations of the CR register so we'll rename them to be unique. -->
  <xsl:param name="cr1_replacement" select="'CR_LIN'"/>
  <xsl:template match="/device/peripherals/peripheral[name='USART0']/registers/register[name='CR'][1]/name/text()">
    <xsl:value-of select="$cr1_replacement"/>
  </xsl:template>

  <xsl:param name="cr2_replacement" select="'CR_SPI'"/>
  <xsl:template match="/device/peripherals/peripheral[name='USART0']/registers/register[name='CR'][2]/name/text()">
    <xsl:value-of select="$cr2_replacement"/>
  </xsl:template>

  <xsl:param name="cr3_replacement" select="'CR_USART'"/>
  <xsl:template match="/device/peripherals/peripheral[name='USART0']/registers/register[name='CR'][3]/name/text()">
    <xsl:value-of select="$cr3_replacement"/>
  </xsl:template>

  <!-- The USART0 peripheral has different representations of the CSR register so we'll rename them to be unique. -->
  <xsl:param name="csr1_replacement" select="'CSR_LIN'"/>
  <xsl:template match="/device/peripherals/peripheral[name='USART0']/registers/register[name='CSR'][1]/name/text()">
    <xsl:value-of select="$csr1_replacement"/>
  </xsl:template>

  <xsl:param name="csr2_replacement" select="'CSR_SPI'"/>
  <xsl:template match="/device/peripherals/peripheral[name='USART0']/registers/register[name='CSR'][2]/name/text()">
    <xsl:value-of select="$csr2_replacement"/>
  </xsl:template>

  <xsl:param name="csr3_replacement" select="'CSR_USART'"/>
  <xsl:template match="/device/peripherals/peripheral[name='USART0']/registers/register[name='CSR'][3]/name/text()">
    <xsl:value-of select="$csr3_replacement"/>
  </xsl:template>

  <!-- The USART0 peripheral has different representations of the IDR register so we'll rename them to be unique. -->
  <xsl:param name="idr1_replacement" select="'IDR_LIN'"/>
  <xsl:template match="/device/peripherals/peripheral[name='USART0']/registers/register[name='IDR'][1]/name/text()">
    <xsl:value-of select="$idr1_replacement"/>
  </xsl:template>

  <xsl:param name="idr2_replacement" select="'IDR_SPI'"/>
  <xsl:template match="/device/peripherals/peripheral[name='USART0']/registers/register[name='IDR'][2]/name/text()">
    <xsl:value-of select="$idr2_replacement"/>
  </xsl:template>

  <xsl:param name="idr3_replacement" select="'IDR_USART'"/>
  <xsl:template match="/device/peripherals/peripheral[name='USART0']/registers/register[name='IDR'][3]/name/text()">
    <xsl:value-of select="$idr3_replacement"/>
  </xsl:template>

  <!-- The USART0 peripheral has different representations of the IER register so we'll rename them to be unique. -->
  <xsl:param name="ier1_replacement" select="'IER_LIN'"/>
  <xsl:template match="/device/peripherals/peripheral[name='USART0']/registers/register[name='IER'][1]/name/text()">
    <xsl:value-of select="$ier1_replacement"/>
  </xsl:template>

  <xsl:param name="ier2_replacement" select="'IER_SPI'"/>
  <xsl:template match="/device/peripherals/peripheral[name='USART0']/registers/register[name='IER'][2]/name/text()">
    <xsl:value-of select="$ier2_replacement"/>
  </xsl:template>

  <xsl:param name="ier3_replacement" select="'IER_USART'"/>
  <xsl:template match="/device/peripherals/peripheral[name='USART0']/registers/register[name='IER'][3]/name/text()">
    <xsl:value-of select="$ier3_replacement"/>
  </xsl:template>

  <!-- The USART0 peripheral has different representations of the IMR register so we'll rename them to be unique. -->
  <xsl:param name="imr1_replacement" select="'IMR_LIN'"/>
  <xsl:template match="/device/peripherals/peripheral[name='USART0']/registers/register[name='IMR'][1]/name/text()">
    <xsl:value-of select="$imr1_replacement"/>
  </xsl:template>

  <xsl:param name="imr2_replacement" select="'IMR_SPI'"/>
  <xsl:template match="/device/peripherals/peripheral[name='USART0']/registers/register[name='IMR'][2]/name/text()">
    <xsl:value-of select="$imr2_replacement"/>
  </xsl:template>

  <xsl:param name="imr3_replacement" select="'IMR_USART'"/>
  <xsl:template match="/device/peripherals/peripheral[name='USART0']/registers/register[name='IMR'][3]/name/text()">
    <xsl:value-of select="$imr3_replacement"/>
  </xsl:template>

  <!-- The USART0 peripheral has different representations of the MR register so we'll rename them to be unique. -->
  <!-- NOTE: There's no MR register for LIN mode in the SVD -->
  <xsl:param name="mr1_replacement" select="'MR_SPI'"/>
  <xsl:template match="/device/peripherals/peripheral[name='USART0']/registers/register[name='MR'][1]/name/text()">
    <xsl:value-of select="$mr1_replacement"/>
  </xsl:template>

  <xsl:param name="mr2_replacement" select="'MR_USART'"/>
  <xsl:template match="/device/peripherals/peripheral[name='USART0']/registers/register[name='IMR'][2]/name/text()">
    <xsl:value-of select="$mr2_replacement"/>
  </xsl:template>
</xsl:stylesheet>
