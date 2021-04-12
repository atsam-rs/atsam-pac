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
</xsl:stylesheet>
