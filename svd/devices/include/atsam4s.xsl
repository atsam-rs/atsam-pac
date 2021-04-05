<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <!-- The CHIPID enumerations for the SAM4SxA and SAM3SxA (and SAM4SxB/SAM3SxB and SAM4SxC/SAM3SxC) are the same.  We remove the
  second enumerated values here since the two chips share the same IDs and created duplicates in the created enumeration.
  -->
  <!--
  <xsl:template match="/device/peripherals/peripheral[name='CHIPID']/registers/register[name='CIDR']/fields/field[name='ARCH']/enumeratedValues/enumeratedValue[name='SAM4SxA']">
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='CHIPID']/registers/register[name='CIDR']/fields/field[name='ARCH']/enumeratedValues/enumeratedValue[name='SAM4SxB']">
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='CHIPID']/registers/register[name='CIDR']/fields/field[name='ARCH']/enumeratedValues/enumeratedValue[name='SAM4SxC']">
  </xsl:template>
  -->
</xsl:stylesheet>
