#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "binary"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"binary\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"maximum\": 1.0,"]
#[doc = "  \"minimum\": 0.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Binary(pub i64);
impl ::std::ops::Deref for Binary {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<Binary> for i64 {
    fn from(value: Binary) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Binary> for Binary {
    fn from(value: &Binary) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for Binary {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Binary {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Binary {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for Binary {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Binary {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for Binary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "Common Transmission Model (CTM) Data Schema v0.1"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"CTM Data\","]
#[doc = "  \"description\": \"Common Transmission Model (CTM) Data Schema v0.1\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"ctm_version\","]
#[doc = "    \"network\","]
#[doc = "    \"temporal_boundary\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ctm_version\": {"]
#[doc = "      \"description\": \"release version of CTM specification\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"network\": {"]
#[doc = "      \"description\": \"structure to hold persistent network data\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"area\","]
#[doc = "        \"bus\","]
#[doc = "        \"gen\","]
#[doc = "        \"global_params\","]
#[doc = "        \"load\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"ac_line\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"structure to hold ac line data using concentrated (6-parameter circuit) PI model\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"bus_fr\","]
#[doc = "              \"bus_to\","]
#[doc = "              \"r\","]
#[doc = "              \"status\","]
#[doc = "              \"uid\","]
#[doc = "              \"x\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"b_fr\": {"]
#[doc = "                \"description\": \"[S or pu] shunt susceptance of line at from terminal\","]
#[doc = "                \"default\": 0,"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"b_to\": {"]
#[doc = "                \"description\": \"[S or pu] shunt susceptance of line at to terminal\","]
#[doc = "                \"default\": 0,"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"bus_fr\": {"]
#[doc = "                \"description\": \"uid of bus at the from terminal of ac line\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"bus_to\": {"]
#[doc = "                \"description\": \"uid of bus at the to terminal of ac line\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"cm_ub_a\": {"]
#[doc = "                \"description\": \"[kA or pu] persistent current rating\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"cm_ub_b\": {"]
#[doc = "                \"description\": \"[kA or pu] 4-hour current rating\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"cm_ub_c\": {"]
#[doc = "                \"description\": \"[kA or pu] 15-minute current rating\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional ac line parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"g_fr\": {"]
#[doc = "                \"description\": \"[S or pu] shunt conductance of line at from terminal\","]
#[doc = "                \"default\": 0,"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"g_to\": {"]
#[doc = "                \"description\": \"[S or pu] shunt conductance of line at to terminal\","]
#[doc = "                \"default\": 0,"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"name\": {"]
#[doc = "                \"description\": \"line name\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"nominal_mva\": {"]
#[doc = "                \"description\": \"[MVA] nominal apparent power of ac line\","]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              },"]
#[doc = "              \"persistent_outage_duration\": {"]
#[doc = "                \"description\": \"[hours] expected duration of persistent outage (time between outage and crews re-energizing the branch)\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"persistent_outage_rate\": {"]
#[doc = "                \"description\": \"[events/year] number of expected persistent outages per year (outages not cleared by reconnectors)\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"r\": {"]
#[doc = "                \"description\": \"[Ohm or pu] series resistance of line\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"sm_ub_a\": {"]
#[doc = "                \"description\": \"[MVA or pu] persistent apparent power rating\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"sm_ub_b\": {"]
#[doc = "                \"description\": \"[MVA or pu] 4-hour apparent power rating\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"sm_ub_c\": {"]
#[doc = "                \"description\": \"[MVA or pu] 15-minute apparent power rating\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"status\": {"]
#[doc = "                \"$ref\": \"#/$defs/status\""]
#[doc = "              },"]
#[doc = "              \"transient_outage_rate\": {"]
#[doc = "                \"description\": \"[events/year] number of expected transient outages per year (outages cleared by reconnectors)\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"vad_lb\": {"]
#[doc = "                \"description\": \"[deg] voltage angle difference lower bound (stability)\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"vad_ub\": {"]
#[doc = "                \"description\": \"[deg] voltage angle difference upper bound (stability)\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"x\": {"]
#[doc = "                \"description\": \"[Ohm or pu] series impedance of line\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"area\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"geographical subset of the electrical network with common Automatic Generation Control (AGC) and responsible for its Area Control Error (ACE)\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"status\","]
#[doc = "              \"uid\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional area parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"name\": {"]
#[doc = "                \"description\": \"area name\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"status\": {"]
#[doc = "                \"description\": \"binary indicator of whether area should be included or omitted (if omitted all elements within area should be omitted); 1=>included, 0=>omitted\","]
#[doc = "                \"$ref\": \"#/$defs/status\""]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"bus\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"structure to hold bus data\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"base_kv\","]
#[doc = "              \"status\","]
#[doc = "              \"uid\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"area\": {"]
#[doc = "                \"description\": \"uid for area to which bus belongs to\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"base_kv\": {"]
#[doc = "                \"description\": \"bus base (nominal) voltage\","]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              },"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional bus parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"name\": {"]
#[doc = "                \"description\": \"bus name\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"status\": {"]
#[doc = "                \"$ref\": \"#/$defs/status\""]
#[doc = "              },"]
#[doc = "              \"type\": {"]
#[doc = "                \"description\": \"bus type for power flow calculations (PV, PQ, or slack)\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"string\","]
#[doc = "                    \"enum\": ["]
#[doc = "                      \"PQ\","]
#[doc = "                      \"PV\","]
#[doc = "                      \"slack\""]
#[doc = "                    ]"]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"vm_lb\": {"]
#[doc = "                \"description\": \"bus voltage lower bound\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/positive_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"vm_ub\": {"]
#[doc = "                \"description\": \"bus voltage upper bound\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/positive_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"zone\": {"]
#[doc = "                \"description\": \"uid for zone to which bus belongs to\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"gen\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"structure to hold generator data\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"bus\","]
#[doc = "              \"status\","]
#[doc = "              \"uid\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"bus\": {"]
#[doc = "                \"description\": \"uid of bus to which generator is connected to\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"cost_pg_model\": {"]
#[doc = "                \"description\": \"type of generation cost model (i.e., function translating power/energy to money); POLYNOMIAL => cost_pg_parameters is an array with n+1 coefficients <a_i> for f(x) = a_0 + a_1 x^1 + ... + a_n x^n; PIECEWISE_LINEAR => cost_pg_parameters is a series of values <x_i, f_i> and cost (f) should be interpolated linearly in between points; MARGINAL_COST => cost_pg_parameters is a series of values <b_i, m_i>, where m_i is a marginal cost ($/MWh or $/(pu*h)) and b_i is the amoung of power (MWh or pu*h) sold at marginal cost m_i\","]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"enum\": ["]
#[doc = "                  \"POLYNOMIAL\","]
#[doc = "                  \"PIECEWISE_LINEAR\","]
#[doc = "                  \"MARGINAL_COST\""]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"cost_pg_parameters\": {"]
#[doc = "                \"description\": \"parameters of generation cost function, can be time dependent\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"array\","]
#[doc = "                    \"items\": {"]
#[doc = "                      \"type\": \"number\""]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/xy_pairs\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"down_time_lb\": {"]
#[doc = "                \"description\": \"[h] minimim time the unit can be out of service (a.k.a., minimum down time)\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional gen parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"forced_outage_rate\": {"]
#[doc = "                \"description\": \"[-] fraction of time the generator is out of service because of forced outages (i.e., hours out of service---because of failures---during a year, divided by 8760)\","]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"maximum\": 1.0,"]
#[doc = "                \"minumum\": 0"]
#[doc = "              },"]
#[doc = "              \"in_service_time_lb\": {"]
#[doc = "                \"description\": \"[h] minimim time the unit can be in service (a.k.a., minimum up time)\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"in_service_time_ub\": {"]
#[doc = "                \"description\": \"[h] maximum time the unit can be in service (commitment == 1)\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"mean_time_to_failure\": {"]
#[doc = "                \"description\": \"[h] mean time to occurence of a failure; failures can be assumed to follow a Poisson process\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"mean_time_to_repair\": {"]
#[doc = "                \"description\": \"[h] mean time to repair a failure\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"name\": {"]
#[doc = "                \"description\": \"generator name\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"nominal_mva\": {"]
#[doc = "                \"description\": \"[MVA] nominal apparent power of generator (nameplate capacity)\","]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              },"]
#[doc = "              \"pg_delta_lb\": {"]
#[doc = "                \"description\": \"[MW/h or pu/h] maximum active power decrease per hour\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"pg_delta_ub\": {"]
#[doc = "                \"description\": \"[MW/h or pu/h] maximum active power increase per hour\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"pg_lb\": {"]
#[doc = "                \"description\": \"[MW or pu] lower bound of active power injection (rectangular operating zone)\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"pg_ub\": {"]
#[doc = "                \"description\": \"[MW or pu] upper bound of active power injection (rectangular operating zone)\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"primary_source\": {"]
#[doc = "                \"description\": \"primary energy source\","]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"enum\": ["]
#[doc = "                  \"COAL\","]
#[doc = "                  \"OIL\","]
#[doc = "                  \"GAS\","]
#[doc = "                  \"NUCLEAR\","]
#[doc = "                  \"BIOMASS\","]
#[doc = "                  \"GEOTHERMAL\","]
#[doc = "                  \"SOLAR\","]
#[doc = "                  \"WIND\","]
#[doc = "                  \"HYDRO\","]
#[doc = "                  \"OTHER\""]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"primary_source_subtype\": {"]
#[doc = "                \"description\": \"subtype of primary energy source; thermal classification taken from https://www.eia.gov/survey/form/eia_923/instructions.pdf\","]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"enum\": ["]
#[doc = "                  \"ANTRHC_BITMN_COAL\","]
#[doc = "                  \"WASTE_COAL\","]
#[doc = "                  \"DISTILLATE_FUEL_OIL\","]
#[doc = "                  \"WASTE_OIL\","]
#[doc = "                  \"PETROLEUM_COKE\","]
#[doc = "                  \"RESIDUAL_FUEL_OIL\","]
#[doc = "                  \"NATURAL_GAS\","]
#[doc = "                  \"OTHER_GAS\","]
#[doc = "                  \"NUCLEAR\","]
#[doc = "                  \"AG_BIPRODUCT\","]
#[doc = "                  \"MUNICIPAL_WASTE\","]
#[doc = "                  \"WOOD_WASTE\","]
#[doc = "                  \"GEOTHERMAL\","]
#[doc = "                  \"SOLAR_PV\","]
#[doc = "                  \"SOLAR_CSP\","]
#[doc = "                  \"WIND_ONSHORE\","]
#[doc = "                  \"WIND_OFFSHORE\","]
#[doc = "                  \"HYDRO_RUN_OF_THE_RIVER\","]
#[doc = "                  \"HYDRO_DAM\","]
#[doc = "                  \"HYDRO_PUMPED_STORAGE\","]
#[doc = "                  \"OTHER\""]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"qg_lb\": {"]
#[doc = "                \"description\": \"[MVAr or pu] lower bound of reactive power injection (rectangular operating zone)\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"qg_ub\": {"]
#[doc = "                \"description\": \"[MVAr or pu] upper bound of reactive power injection (rectangular operating zone)\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"scheduled_maintenance_rate\": {"]
#[doc = "                \"description\": \"[-] fraction of time the generator is out of service because of scheduled maintenance (i.e., hours out of service---because of scheduled maintenance---during a year, divided by 8760)\","]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"maximum\": 1.0,"]
#[doc = "                \"minumum\": 0"]
#[doc = "              },"]
#[doc = "              \"service_required\": {"]
#[doc = "                \"description\": \"whether generator must be in service (e.g., nuclear power plant) or out of service (e.g., generator during maintenance or after an outage); 0 => no requirement, 1 => fixed in service, 2 => fixed out of service\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"integer\","]
#[doc = "                    \"maximum\": 2.0,"]
#[doc = "                    \"minimum\": 0.0"]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"shutdown_cost\": {"]
#[doc = "                \"description\": \"[$] cost of shutting down the unit\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"startup_cost_cold\": {"]
#[doc = "                \"description\": \"[$] cost of starting the unit after being off > startup_time_warm hours\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"startup_cost_hot\": {"]
#[doc = "                \"description\": \"[$] cost of starting the unit after being off <= startup_time_hot hours\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"startup_cost_warm\": {"]
#[doc = "                \"description\": \"[$] cost of starting the unit after being off > startup_time_hot hours, but <= startup_time_warm hours\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"startup_time_hot\": {"]
#[doc = "                \"description\": \"[h] maximum time the unit can be off before a hot startup\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"startup_time_warm\": {"]
#[doc = "                \"description\": \"[h] maximum time the unit can be off before a warm startup\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"status\": {"]
#[doc = "                \"$ref\": \"#/$defs/status\""]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"vm_setpoint\": {"]
#[doc = "                \"description\": \"[kV or pu] target voltage magnitude of the bus that this generator connects to\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/positive_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"global_params\": {"]
#[doc = "          \"description\": \"structure to hold global settings for parameters in the network\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"unit_convention\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"base_mva\": {"]
#[doc = "              \"description\": \"[MVA] system-wide apparent power base\","]
#[doc = "              \"default\": 100.0,"]
#[doc = "              \"$ref\": \"#/$defs/positive_number\""]
#[doc = "            },"]
#[doc = "            \"bus_ref\": {"]
#[doc = "              \"description\": \"UID of reference bus of the electrical network\","]
#[doc = "              \"$ref\": \"#/$defs/uid\""]
#[doc = "            },"]
#[doc = "            \"unit_convention\": {"]
#[doc = "              \"description\": \"units used for physical network parameters\","]
#[doc = "              \"type\": \"string\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"NATURAL_UNITS\","]
#[doc = "                \"PER_UNIT_COMPONENT_BASE\","]
#[doc = "                \"PER_UNIT_SYSTEM_BASE\""]
#[doc = "              ]"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"hvdc_p2p\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"structure to hold point-to-point hvdc line data\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"bus_fr\","]
#[doc = "              \"bus_to\","]
#[doc = "              \"status\","]
#[doc = "              \"uid\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"base_kv_dc\": {"]
#[doc = "                \"description\": \"[kV] base voltage at the dc side\","]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              },"]
#[doc = "              \"bus_fr\": {"]
#[doc = "                \"description\": \"uid of bus at the from terminal of hvdc line\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"bus_to\": {"]
#[doc = "                \"description\": \"uid of bus at the to terminal of hvdc line\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"cm_ub_fr\": {"]
#[doc = "                \"description\": \"[kA or pu] ac persistent current rating, from terminal (if in pu, use from bus base_kv)\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"cm_ub_to\": {"]
#[doc = "                \"description\": \"[kA or pu] ac persistent current rating, to terminal (if in pu, use to bus base_kv)\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional hvdc point-to-point parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"loss_a\": {"]
#[doc = "                \"description\": \"[MW or pu] standby loss\","]
#[doc = "                \"default\": 0,"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"loss_b\": {"]
#[doc = "                \"description\": \"[kV or pu] loss proportional to current magnitude (if in pu, base voltage corresponds to base_kv_dc)\","]
#[doc = "                \"default\": 0,"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"loss_c\": {"]
#[doc = "                \"description\": \"[Ohm or pu] loss proportional to current magnitude squared (if in pu, base voltage corresponds to base_kv_dc)\","]
#[doc = "                \"default\": 0,"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"name\": {"]
#[doc = "                \"description\": \"HVDC line name\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"nominal_mva\": {"]
#[doc = "                \"description\": \"[MVA] nominal apparent power of hvdc line\","]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              },"]
#[doc = "              \"p\": {"]
#[doc = "                \"description\": \"number of poles; 1 => monopole, 2 => bipole\","]
#[doc = "                \"type\": \"integer\","]
#[doc = "                \"maximum\": 2.0,"]
#[doc = "                \"minimum\": 1.0"]
#[doc = "              },"]
#[doc = "              \"pdc_fr_lb\": {"]
#[doc = "                \"description\": \"[MW or pu] minimum active power entering hvdc line at from bus\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"pdc_fr_ub\": {"]
#[doc = "                \"description\": \"[MW or pu] maximum active power entering hvdc line at from bus\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"pdc_to_lb\": {"]
#[doc = "                \"description\": \"[MW or pu] minimum active power entering hvdc line at to bus\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"pdc_to_ub\": {"]
#[doc = "                \"description\": \"[MW or pu] maximum active power entering hvdc line at to bus\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"persistent_outage_duration\": {"]
#[doc = "                \"description\": \"[hours] expected duration of persistent outage (time between outage and crews re-energizing the branch)\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"persistent_outage_rate\": {"]
#[doc = "                \"description\": \"[events/year] number of expected persistent outages per year (outages not cleared by reconnectors)\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"phi_lb\": {"]
#[doc = "                \"description\": \"[deg] only meaningful if technology == LCC; firing angle minimum\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"phi_ub\": {"]
#[doc = "                \"description\": \"[deg] only meaningful if technology == LCC; firing angle maximum\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"qdc_fr_lb\": {"]
#[doc = "                \"description\": \"[MVAr or pu] minimum reactive power entering hvdc line at from bus\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"qdc_fr_ub\": {"]
#[doc = "                \"description\": \"[MVAr or pu] maximum reactive power entering hvdc line at from bus\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"qdc_to_lb\": {"]
#[doc = "                \"description\": \"[MVAr or pu] minimum reactive power entering hvdc line at to bus\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"qdc_to_ub\": {"]
#[doc = "                \"description\": \"[MW or pu] maximum active power entering hvdc line at to bus\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"r\": {"]
#[doc = "                \"description\": \"[Ohm or pu] dc line resistance (if in pu, base voltage corresponds to base_kv_dc)\","]
#[doc = "                \"default\": 0.0,"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"sm_ub\": {"]
#[doc = "                \"description\": \"[MVA or pu] ac persistent apparent power rating\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"status\": {"]
#[doc = "                \"$ref\": \"#/$defs/status\""]
#[doc = "              },"]
#[doc = "              \"technology\": {"]
#[doc = "                \"description\": \"power conversion technology\","]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"enum\": ["]
#[doc = "                  \"LCC\","]
#[doc = "                  \"VSC\","]
#[doc = "                  \"MMC\""]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"transient_outage_rate\": {"]
#[doc = "                \"description\": \"[events/year] number of expected transient outages per year (outages cleared by reconnectors or other)\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"vm_dc_lb\": {"]
#[doc = "                \"description\": \"[kV or pu] minimum voltage at the dc side\","]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              },"]
#[doc = "              \"vm_dc_ub\": {"]
#[doc = "                \"description\": \"[kV or pu] maximum voltage at the dc side\","]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"load\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"structure to hold load (consumer) data using ZIP model\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"bus\","]
#[doc = "              \"pd\","]
#[doc = "              \"qd\","]
#[doc = "              \"status\","]
#[doc = "              \"uid\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"bus\": {"]
#[doc = "                \"description\": \"uid of bus to which load is connected to\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional bus parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"name\": {"]
#[doc = "                \"description\": \"load name\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"nominal_mva\": {"]
#[doc = "                \"description\": \"[MVA] nominal power of load\","]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              },"]
#[doc = "              \"pd\": {"]
#[doc = "                \"description\": \"active power demand\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"pd_i\": {"]
#[doc = "                \"description\": \"constant current active power demand at v_bus = 1.0 pu\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"pd_y\": {"]
#[doc = "                \"description\": \"constant impedance active power demand at v_bus = 1.0 pu\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"qd\": {"]
#[doc = "                \"description\": \"reactive power demand\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"qd_i\": {"]
#[doc = "                \"description\": \"constant current reactive power demand at v_bus = 1.0 pu\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"qd_y\": {"]
#[doc = "                \"description\": \"constant impedance reactive power demand at v_bus = 1.0 pu\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"status\": {"]
#[doc = "                \"$ref\": \"#/$defs/status\""]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"reserve\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"structure to hold reserve product and requirement data\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"reserve_type\","]
#[doc = "              \"status\","]
#[doc = "              \"uid\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional reserve parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"name\": {"]
#[doc = "                \"description\": \"name of reserve product\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"participants\": {"]
#[doc = "                \"description\": \"uid of generators contributing to this reserve\","]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"$ref\": \"#/$defs/uid\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"pg_down\": {"]
#[doc = "                \"description\": \"[MW or pu] downward active power required by this reserve\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"pg_up\": {"]
#[doc = "                \"description\": \"[MW or pu] upward active power required by this reserve\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"reserve_type\": {"]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"enum\": ["]
#[doc = "                  \"PRIMARY\","]
#[doc = "                  \"SECONDARY\","]
#[doc = "                  \"TERTIARY\""]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"status\": {"]
#[doc = "                \"$ref\": \"#/$defs/status\""]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"shunt\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"structure to hold shunt data\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"bs\","]
#[doc = "              \"bus\","]
#[doc = "              \"gs\","]
#[doc = "              \"num_steps_ub\","]
#[doc = "              \"status\","]
#[doc = "              \"uid\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"bs\": {"]
#[doc = "                \"description\": \"[MVAr or pu] reactive power demand at v_bus = 1.0 pu, per step of each shunt section\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"type\": \"array\","]
#[doc = "                    \"items\": {"]
#[doc = "                      \"type\": \"number\""]
#[doc = "                    }"]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"bus\": {"]
#[doc = "                \"description\": \"uid of bus to which shunt is connected to\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional shunt parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"gs\": {"]
#[doc = "                \"description\": \"[MW or pu] active power demand at v_bus = 1.0 pu, per step of each shunt section\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"type\": \"array\","]
#[doc = "                    \"items\": {"]
#[doc = "                      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                    }"]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"name\": {"]
#[doc = "                \"description\": \"shunt name\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"nominal_mva\": {"]
#[doc = "                \"description\": \"[MVA] nominal apparent power of shunt (nameplate capacity)\","]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              },"]
#[doc = "              \"num_steps_ub\": {"]
#[doc = "                \"description\": \"upper bound for number of energized steps of shunt section (lower bound is always 0)\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_integer\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"type\": \"array\","]
#[doc = "                    \"items\": {"]
#[doc = "                      \"$ref\": \"#/$defs/nonnegative_integer\""]
#[doc = "                    }"]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"status\": {"]
#[doc = "                \"$ref\": \"#/$defs/status\""]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"storage\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"structure to hold storage (battery) data\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"bus\","]
#[doc = "              \"charge_efficiency\","]
#[doc = "              \"discharge_efficiency\","]
#[doc = "              \"ps_ex\","]
#[doc = "              \"qs_ex\","]
#[doc = "              \"status\","]
#[doc = "              \"uid\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"bus\": {"]
#[doc = "                \"description\": \"uid of bus to which generator is connected to\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"charge_efficiency\": {"]
#[doc = "                \"description\": \"[-] charge efficiency, in (0, 1]\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\","]
#[doc = "                    \"maximum\": 1.0,"]
#[doc = "                    \"exclusiveMinimum\": 0.0"]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"charge_ub\": {"]
#[doc = "                \"description\": \"[MW or pu] maximum rate of charge\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"cm_ub\": {"]
#[doc = "                \"description\": \"[kA or pu] converter current output rating\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"discharge_efficiency\": {"]
#[doc = "                \"description\": \"[-] discharge efficiency, in (0, 1]\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\","]
#[doc = "                    \"maximum\": 1.0,"]
#[doc = "                    \"exclusiveMinimum\": 0.0"]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"discharge_ub\": {"]
#[doc = "                \"description\": \"[MW or pu] maximum rate of discharge\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"energy_ub\": {"]
#[doc = "                \"description\": \"[MWh or pu*h] maximum state of charge\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional storage parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"name\": {"]
#[doc = "                \"description\": \"storage name\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"nominal_mva\": {"]
#[doc = "                \"description\": \"[MVA] nominal apparent power of storage (nameplate capacity)\","]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              },"]
#[doc = "              \"ps_delta_lb\": {"]
#[doc = "                \"description\": \"[MW/h or pu/h] maximum active power decrease per hour\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"ps_delta_ub\": {"]
#[doc = "                \"description\": \"[MW/h or pu/h] maximum active power increase per hour\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"ps_ex\": {"]
#[doc = "                \"description\": \"converter standby active power exogenous draw\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"qs_ex\": {"]
#[doc = "                \"description\": \"converter standby reactive power exogenous draw\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"qs_lb\": {"]
#[doc = "                \"description\": \"[MVAr or pu] minumum reactive power injection\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"qs_ub\": {"]
#[doc = "                \"description\": \"[MVAr or pu] maximum reactive power injection\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"sm_ub\": {"]
#[doc = "                \"description\": \"[MVA or pu] converter apparent power rating\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"status\": {"]
#[doc = "                \"$ref\": \"#/$defs/status\""]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"switch\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"bus_fr\","]
#[doc = "              \"bus_to\","]
#[doc = "              \"status\","]
#[doc = "              \"uid\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"bus_fr\": {"]
#[doc = "                \"description\": \"uid of bus at the from terminal of switch\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"bus_to\": {"]
#[doc = "                \"description\": \"uid of bus at the to terminal of switch\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"cm_ub\": {"]
#[doc = "                \"description\": \"[kA or pu] current limit\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional switch parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"name\": {"]
#[doc = "                \"description\": \"name of switch\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"nominal_mva\": {"]
#[doc = "                \"description\": \"[MVA] nominal apparent power of switch (nameplate capacity)\","]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              },"]
#[doc = "              \"sm_ub\": {"]
#[doc = "                \"description\": \"[MVA or pu] apparent power flow limit\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"status\": {"]
#[doc = "                \"$ref\": \"#/$defs/status\""]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"transformer\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"structure to hold 2-winding transformer and phase shifter data using simplified (4-parameter circuit) model\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"b\","]
#[doc = "              \"bus_fr\","]
#[doc = "              \"bus_to\","]
#[doc = "              \"g\","]
#[doc = "              \"r\","]
#[doc = "              \"status\","]
#[doc = "              \"uid\","]
#[doc = "              \"x\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"b\": {"]
#[doc = "                \"description\": \"[S or pu] shunt susceptance of transformer at from terminal (magnetizing branch)\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"bus_fr\": {"]
#[doc = "                \"description\": \"uid of bus at the from terminal of transformer\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"bus_to\": {"]
#[doc = "                \"description\": \"uid of bus at the to terminal of transformer\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"cm_ub_a\": {"]
#[doc = "                \"description\": \"[kA or pu] persistent current rating, referred to from side\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"cm_ub_b\": {"]
#[doc = "                \"description\": \"[kA or pu] 4-hour current rating, referred to from side\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"cm_ub_c\": {"]
#[doc = "                \"description\": \"[kA or pu] 15-minute current rating, referred to from side\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional transformer parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"g\": {"]
#[doc = "                \"description\": \"[S or pu] shunt conductance of transformer at from terminal (magnetizing branch)\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"name\": {"]
#[doc = "                \"description\": \"transformer name\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"nominal_mva\": {"]
#[doc = "                \"description\": \"[MVA] nominal apparent power of transformer\","]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              },"]
#[doc = "              \"persistent_outage_duration\": {"]
#[doc = "                \"description\": \"[hours] expected duration of persistent outage (time between outage and crews re-energizing the branch)\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"persistent_outage_rate\": {"]
#[doc = "                \"description\": \"[events/year] number of expected persistent outages per year (outages not cleared by reconnectors)\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"r\": {"]
#[doc = "                \"description\": \"[Ohm or pu] series resistance of line\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"sm_ub_a\": {"]
#[doc = "                \"description\": \"[MVA or pu] persistent apparent power rating, referred to from side\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"sm_ub_b\": {"]
#[doc = "                \"description\": \"[MVA or pu] 4-hour apparent power rating, referred to from side\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"sm_ub_c\": {"]
#[doc = "                \"description\": \"[MVA or pu] 15-minute apparent power rating, referred to from side\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"status\": {"]
#[doc = "                \"$ref\": \"#/$defs/status\""]
#[doc = "              },"]
#[doc = "              \"ta_lb\": {"]
#[doc = "                \"description\": \"[deg] minimum angle phase shift (angle difference = va_from - va_to - angle_shift)\","]
#[doc = "                \"default\": 0,"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"ta_steps\": {"]
#[doc = "                \"description\": \"number of discrete steps between ta_lb and ta_ub (including limit values)\","]
#[doc = "                \"default\": 1,"]
#[doc = "                \"$ref\": \"#/$defs/positive_integer\""]
#[doc = "              },"]
#[doc = "              \"ta_ub\": {"]
#[doc = "                \"description\": \"[deg] maximum angle phase shift (angle difference = va_from - va_to - angle_shift)\","]
#[doc = "                \"default\": 0,"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"tm_lb\": {"]
#[doc = "                \"description\": \"[-] minimum tap ratio (1.0 correspond to nominal ratio, inner_vm_from = vm_from * tap_value)\","]
#[doc = "                \"default\": 1.0,"]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              },"]
#[doc = "              \"tm_steps\": {"]
#[doc = "                \"description\": \"number of discrete steps between tm_lb and tm_ub (including limit values)\","]
#[doc = "                \"default\": 1,"]
#[doc = "                \"$ref\": \"#/$defs/positive_integer\""]
#[doc = "              },"]
#[doc = "              \"tm_ub\": {"]
#[doc = "                \"description\": \"[-] maximum tap ratio (1.0 correspond to nominal ratio, inner_vm_from = vm_from * tap_value)\","]
#[doc = "                \"default\": 1.0,"]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"x\": {"]
#[doc = "                \"description\": \"[Ohm or pu] series impedance of line\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"zone\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"geographical subset of the electrical network commonly associated with market purposes (e.g., define sub-markets within a large interconnected system, defining different areas for reserve products, etc.)\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"status\","]
#[doc = "              \"uid\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional zone parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"name\": {"]
#[doc = "                \"description\": \"zone name\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"status\": {"]
#[doc = "                \"description\": \"binary indicator of whether zone should be included or omitted (if omitted all elements within zone should be omitted); 1=>included, 0=>omitted\","]
#[doc = "                \"$ref\": \"#/$defs/status\""]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"temporal_boundary\": {"]
#[doc = "      \"description\": \"structure to hold data on initial conditions of power system (state prior to start of time series data)\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"global_params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"bus\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"structure to hold initial state of bus variables\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"uid\","]
#[doc = "              \"va\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional bus initial condition parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"description\": \"uid of bus this record refers to\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"va\": {"]
#[doc = "                \"description\": \"[deg] initial voltage angle\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"vm\": {"]
#[doc = "                \"description\": \"[kV or pu] initial voltage magnitude\","]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"gen\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"structure to hold initial state of generator variables\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"pg\","]
#[doc = "              \"uid\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"down_time\": {"]
#[doc = "                \"description\": \"[h] if in service, zero, else time the unit has been out of service\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional generator initial condition parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"in_service_time\": {"]
#[doc = "                \"description\": \"[h] if in service, time the unit has been in service, zero otherwise\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"pg\": {"]
#[doc = "                \"description\": \"[MW or pu] initial active power injection\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"qg\": {"]
#[doc = "                \"description\": \"[MW or pu] initial reactive power injection\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"description\": \"uid of generator this record refers to\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"global_params\": {"]
#[doc = "          \"description\": \"structure to hold global parameters of temporal boundary\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"time_elapsed\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"time_elapsed\": {"]
#[doc = "              \"description\": \"[seconds] time elapsed since temporal_boundary conditions where present in the system\","]
#[doc = "              \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"hvdc_p2p\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"structure to hold initial state of hvdc point-to-point line variables\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"pdc_fr\","]
#[doc = "              \"pdc_to\","]
#[doc = "              \"uid\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional hvdc point-to-point line initial condition parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"pdc_fr\": {"]
#[doc = "                \"description\": \"[MW or pu] initial active power entering hvdc line at from bus\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"pdc_to\": {"]
#[doc = "                \"description\": \"[MW or pu] initial active power entering hvdc line at to bus\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"qdc_fr\": {"]
#[doc = "                \"description\": \"[MVAr or pu] initial reactive power entering hvdc line at from bus\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"qdc_to\": {"]
#[doc = "                \"description\": \"[MVAr or pu] initial reactive power entering hvdc line at to bus\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"description\": \"uid of hvdc point-to-point this record refers to\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              },"]
#[doc = "              \"vm_dc_fr\": {"]
#[doc = "                \"description\": \"[kV or pu] initial dc side voltage at from converter\","]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              },"]
#[doc = "              \"vm_dc_to\": {"]
#[doc = "                \"description\": \"[kV or pu] initial dc side voltage at to converter\","]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"shunt\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"structure to hold initial state of shunt variables\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"num_steps\","]
#[doc = "              \"uid\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional shunt initial condition parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"num_steps\": {"]
#[doc = "                \"description\": \"[-] number of initial energized steps per section\","]
#[doc = "                \"anyOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/$defs/nonnegative_integer\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"type\": \"array\","]
#[doc = "                    \"items\": {"]
#[doc = "                      \"$ref\": \"#/$defs/nonnegative_integer\""]
#[doc = "                    }"]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"description\": \"uid of shunt this record refers to\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"storage\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"structure to hold initial state of storage variables\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"energy\","]
#[doc = "              \"uid\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"energy\": {"]
#[doc = "                \"description\": \"[MWh or pu*h] initial state of charge\","]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional storage initial condition parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"ps\": {"]
#[doc = "                \"description\": \"[MW or pu] initial active power injection\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"qs\": {"]
#[doc = "                \"description\": \"[MW or pu] initial reactive power injection\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"description\": \"uid of storage this record refers to\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"switch\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"structure to hold initial state of switch variables\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"state\","]
#[doc = "              \"uid\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional switch initial condition parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"state\": {"]
#[doc = "                \"description\": \"[-] binary indicator of switch initial status; 0 => open, 1 => closed\","]
#[doc = "                \"$ref\": \"#/$defs/binary\""]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"description\": \"uid of switch this record refers to\","]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"transformer\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"structure to hold initial state of transformer variables\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"ta\","]
#[doc = "              \"tm\","]
#[doc = "              \"uid\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"ext\": {"]
#[doc = "                \"description\": \"additional transformer initial condition parameters currently not supported by CTM\""]
#[doc = "              },"]
#[doc = "              \"ta\": {"]
#[doc = "                \"description\": \"[deg] initial angle phase shift\","]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"tm\": {"]
#[doc = "                \"description\": \"[-] initial tap ratio\","]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              },"]
#[doc = "              \"uid\": {"]
#[doc = "                \"$ref\": \"#/$defs/uid\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"time_series_data\": {"]
#[doc = "      \"description\": \"structure to contain all time variant data of the system/case. All time series are synchronized to the same timestamps, which should should be stored using Unix time. Structure is quasi-tabular, with uid, name, path_to_file, values, and ext being arrays in the same order of said field. This is done in order to allow for better compression (e.g., using HDF5) for the values field.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"uid\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"ext\": {"]
#[doc = "          \"description\": \"additional time series information not currently supported by CTM\","]
#[doc = "          \"type\": \"array\""]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"description\": \"array of names of time series\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"path_to_file\": {"]
#[doc = "          \"description\": \"path to file containing all time series information or a separate path for each time series\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"timestamp\": {"]
#[doc = "          \"description\": \"[seconds] seconds since epoch (Unix time) for each instant for which time series values are provided\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"uid\": {"]
#[doc = "          \"description\": \"array of uids of time series\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"values\": {"]
#[doc = "          \"description\": \"array of time series values\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"time series values for a particular time series\","]
#[doc = "            \"type\": \"array\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmData {
    #[doc = "release version of CTM specification"]
    pub ctm_version: ::std::string::String,
    pub network: CtmDataNetwork,
    pub temporal_boundary: CtmDataTemporalBoundary,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub time_series_data: ::std::option::Option<CtmDataTimeSeriesData>,
}
impl ::std::convert::From<&CtmData> for CtmData {
    fn from(value: &CtmData) -> Self {
        value.clone()
    }
}
impl CtmData {
    pub fn builder() -> builder::CtmData {
        Default::default()
    }
}
#[doc = "structure to hold persistent network data"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold persistent network data\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"area\","]
#[doc = "    \"bus\","]
#[doc = "    \"gen\","]
#[doc = "    \"global_params\","]
#[doc = "    \"load\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ac_line\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"structure to hold ac line data using concentrated (6-parameter circuit) PI model\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"bus_fr\","]
#[doc = "          \"bus_to\","]
#[doc = "          \"r\","]
#[doc = "          \"status\","]
#[doc = "          \"uid\","]
#[doc = "          \"x\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"b_fr\": {"]
#[doc = "            \"description\": \"[S or pu] shunt susceptance of line at from terminal\","]
#[doc = "            \"default\": 0,"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"b_to\": {"]
#[doc = "            \"description\": \"[S or pu] shunt susceptance of line at to terminal\","]
#[doc = "            \"default\": 0,"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"bus_fr\": {"]
#[doc = "            \"description\": \"uid of bus at the from terminal of ac line\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"bus_to\": {"]
#[doc = "            \"description\": \"uid of bus at the to terminal of ac line\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"cm_ub_a\": {"]
#[doc = "            \"description\": \"[kA or pu] persistent current rating\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"cm_ub_b\": {"]
#[doc = "            \"description\": \"[kA or pu] 4-hour current rating\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"cm_ub_c\": {"]
#[doc = "            \"description\": \"[kA or pu] 15-minute current rating\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional ac line parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"g_fr\": {"]
#[doc = "            \"description\": \"[S or pu] shunt conductance of line at from terminal\","]
#[doc = "            \"default\": 0,"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"g_to\": {"]
#[doc = "            \"description\": \"[S or pu] shunt conductance of line at to terminal\","]
#[doc = "            \"default\": 0,"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"line name\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"nominal_mva\": {"]
#[doc = "            \"description\": \"[MVA] nominal apparent power of ac line\","]
#[doc = "            \"$ref\": \"#/$defs/positive_number\""]
#[doc = "          },"]
#[doc = "          \"persistent_outage_duration\": {"]
#[doc = "            \"description\": \"[hours] expected duration of persistent outage (time between outage and crews re-energizing the branch)\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"persistent_outage_rate\": {"]
#[doc = "            \"description\": \"[events/year] number of expected persistent outages per year (outages not cleared by reconnectors)\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"r\": {"]
#[doc = "            \"description\": \"[Ohm or pu] series resistance of line\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"sm_ub_a\": {"]
#[doc = "            \"description\": \"[MVA or pu] persistent apparent power rating\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"sm_ub_b\": {"]
#[doc = "            \"description\": \"[MVA or pu] 4-hour apparent power rating\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"sm_ub_c\": {"]
#[doc = "            \"description\": \"[MVA or pu] 15-minute apparent power rating\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"status\": {"]
#[doc = "            \"$ref\": \"#/$defs/status\""]
#[doc = "          },"]
#[doc = "          \"transient_outage_rate\": {"]
#[doc = "            \"description\": \"[events/year] number of expected transient outages per year (outages cleared by reconnectors)\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"vad_lb\": {"]
#[doc = "            \"description\": \"[deg] voltage angle difference lower bound (stability)\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"vad_ub\": {"]
#[doc = "            \"description\": \"[deg] voltage angle difference upper bound (stability)\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"x\": {"]
#[doc = "            \"description\": \"[Ohm or pu] series impedance of line\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"area\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"geographical subset of the electrical network with common Automatic Generation Control (AGC) and responsible for its Area Control Error (ACE)\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"status\","]
#[doc = "          \"uid\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional area parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"area name\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"status\": {"]
#[doc = "            \"description\": \"binary indicator of whether area should be included or omitted (if omitted all elements within area should be omitted); 1=>included, 0=>omitted\","]
#[doc = "            \"$ref\": \"#/$defs/status\""]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"bus\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"structure to hold bus data\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"base_kv\","]
#[doc = "          \"status\","]
#[doc = "          \"uid\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"area\": {"]
#[doc = "            \"description\": \"uid for area to which bus belongs to\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"base_kv\": {"]
#[doc = "            \"description\": \"bus base (nominal) voltage\","]
#[doc = "            \"$ref\": \"#/$defs/positive_number\""]
#[doc = "          },"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional bus parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"bus name\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"status\": {"]
#[doc = "            \"$ref\": \"#/$defs/status\""]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"description\": \"bus type for power flow calculations (PV, PQ, or slack)\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"enum\": ["]
#[doc = "                  \"PQ\","]
#[doc = "                  \"PV\","]
#[doc = "                  \"slack\""]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"vm_lb\": {"]
#[doc = "            \"description\": \"bus voltage lower bound\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"vm_ub\": {"]
#[doc = "            \"description\": \"bus voltage upper bound\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"zone\": {"]
#[doc = "            \"description\": \"uid for zone to which bus belongs to\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"gen\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"structure to hold generator data\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"bus\","]
#[doc = "          \"status\","]
#[doc = "          \"uid\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"bus\": {"]
#[doc = "            \"description\": \"uid of bus to which generator is connected to\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"cost_pg_model\": {"]
#[doc = "            \"description\": \"type of generation cost model (i.e., function translating power/energy to money); POLYNOMIAL => cost_pg_parameters is an array with n+1 coefficients <a_i> for f(x) = a_0 + a_1 x^1 + ... + a_n x^n; PIECEWISE_LINEAR => cost_pg_parameters is a series of values <x_i, f_i> and cost (f) should be interpolated linearly in between points; MARGINAL_COST => cost_pg_parameters is a series of values <b_i, m_i>, where m_i is a marginal cost ($/MWh or $/(pu*h)) and b_i is the amoung of power (MWh or pu*h) sold at marginal cost m_i\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"POLYNOMIAL\","]
#[doc = "              \"PIECEWISE_LINEAR\","]
#[doc = "              \"MARGINAL_COST\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"cost_pg_parameters\": {"]
#[doc = "            \"description\": \"parameters of generation cost function, can be time dependent\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"type\": \"number\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/xy_pairs\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"down_time_lb\": {"]
#[doc = "            \"description\": \"[h] minimim time the unit can be out of service (a.k.a., minimum down time)\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional gen parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"forced_outage_rate\": {"]
#[doc = "            \"description\": \"[-] fraction of time the generator is out of service because of forced outages (i.e., hours out of service---because of failures---during a year, divided by 8760)\","]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"maximum\": 1.0,"]
#[doc = "            \"minumum\": 0"]
#[doc = "          },"]
#[doc = "          \"in_service_time_lb\": {"]
#[doc = "            \"description\": \"[h] minimim time the unit can be in service (a.k.a., minimum up time)\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"in_service_time_ub\": {"]
#[doc = "            \"description\": \"[h] maximum time the unit can be in service (commitment == 1)\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"mean_time_to_failure\": {"]
#[doc = "            \"description\": \"[h] mean time to occurence of a failure; failures can be assumed to follow a Poisson process\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"mean_time_to_repair\": {"]
#[doc = "            \"description\": \"[h] mean time to repair a failure\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"generator name\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"nominal_mva\": {"]
#[doc = "            \"description\": \"[MVA] nominal apparent power of generator (nameplate capacity)\","]
#[doc = "            \"$ref\": \"#/$defs/positive_number\""]
#[doc = "          },"]
#[doc = "          \"pg_delta_lb\": {"]
#[doc = "            \"description\": \"[MW/h or pu/h] maximum active power decrease per hour\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"pg_delta_ub\": {"]
#[doc = "            \"description\": \"[MW/h or pu/h] maximum active power increase per hour\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"pg_lb\": {"]
#[doc = "            \"description\": \"[MW or pu] lower bound of active power injection (rectangular operating zone)\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"pg_ub\": {"]
#[doc = "            \"description\": \"[MW or pu] upper bound of active power injection (rectangular operating zone)\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"primary_source\": {"]
#[doc = "            \"description\": \"primary energy source\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"COAL\","]
#[doc = "              \"OIL\","]
#[doc = "              \"GAS\","]
#[doc = "              \"NUCLEAR\","]
#[doc = "              \"BIOMASS\","]
#[doc = "              \"GEOTHERMAL\","]
#[doc = "              \"SOLAR\","]
#[doc = "              \"WIND\","]
#[doc = "              \"HYDRO\","]
#[doc = "              \"OTHER\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"primary_source_subtype\": {"]
#[doc = "            \"description\": \"subtype of primary energy source; thermal classification taken from https://www.eia.gov/survey/form/eia_923/instructions.pdf\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"ANTRHC_BITMN_COAL\","]
#[doc = "              \"WASTE_COAL\","]
#[doc = "              \"DISTILLATE_FUEL_OIL\","]
#[doc = "              \"WASTE_OIL\","]
#[doc = "              \"PETROLEUM_COKE\","]
#[doc = "              \"RESIDUAL_FUEL_OIL\","]
#[doc = "              \"NATURAL_GAS\","]
#[doc = "              \"OTHER_GAS\","]
#[doc = "              \"NUCLEAR\","]
#[doc = "              \"AG_BIPRODUCT\","]
#[doc = "              \"MUNICIPAL_WASTE\","]
#[doc = "              \"WOOD_WASTE\","]
#[doc = "              \"GEOTHERMAL\","]
#[doc = "              \"SOLAR_PV\","]
#[doc = "              \"SOLAR_CSP\","]
#[doc = "              \"WIND_ONSHORE\","]
#[doc = "              \"WIND_OFFSHORE\","]
#[doc = "              \"HYDRO_RUN_OF_THE_RIVER\","]
#[doc = "              \"HYDRO_DAM\","]
#[doc = "              \"HYDRO_PUMPED_STORAGE\","]
#[doc = "              \"OTHER\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"qg_lb\": {"]
#[doc = "            \"description\": \"[MVAr or pu] lower bound of reactive power injection (rectangular operating zone)\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"qg_ub\": {"]
#[doc = "            \"description\": \"[MVAr or pu] upper bound of reactive power injection (rectangular operating zone)\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"scheduled_maintenance_rate\": {"]
#[doc = "            \"description\": \"[-] fraction of time the generator is out of service because of scheduled maintenance (i.e., hours out of service---because of scheduled maintenance---during a year, divided by 8760)\","]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"maximum\": 1.0,"]
#[doc = "            \"minumum\": 0"]
#[doc = "          },"]
#[doc = "          \"service_required\": {"]
#[doc = "            \"description\": \"whether generator must be in service (e.g., nuclear power plant) or out of service (e.g., generator during maintenance or after an outage); 0 => no requirement, 1 => fixed in service, 2 => fixed out of service\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"integer\","]
#[doc = "                \"maximum\": 2.0,"]
#[doc = "                \"minimum\": 0.0"]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"shutdown_cost\": {"]
#[doc = "            \"description\": \"[$] cost of shutting down the unit\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"startup_cost_cold\": {"]
#[doc = "            \"description\": \"[$] cost of starting the unit after being off > startup_time_warm hours\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"startup_cost_hot\": {"]
#[doc = "            \"description\": \"[$] cost of starting the unit after being off <= startup_time_hot hours\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"startup_cost_warm\": {"]
#[doc = "            \"description\": \"[$] cost of starting the unit after being off > startup_time_hot hours, but <= startup_time_warm hours\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"startup_time_hot\": {"]
#[doc = "            \"description\": \"[h] maximum time the unit can be off before a hot startup\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"startup_time_warm\": {"]
#[doc = "            \"description\": \"[h] maximum time the unit can be off before a warm startup\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"status\": {"]
#[doc = "            \"$ref\": \"#/$defs/status\""]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"vm_setpoint\": {"]
#[doc = "            \"description\": \"[kV or pu] target voltage magnitude of the bus that this generator connects to\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/positive_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"global_params\": {"]
#[doc = "      \"description\": \"structure to hold global settings for parameters in the network\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"unit_convention\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"base_mva\": {"]
#[doc = "          \"description\": \"[MVA] system-wide apparent power base\","]
#[doc = "          \"default\": 100.0,"]
#[doc = "          \"$ref\": \"#/$defs/positive_number\""]
#[doc = "        },"]
#[doc = "        \"bus_ref\": {"]
#[doc = "          \"description\": \"UID of reference bus of the electrical network\","]
#[doc = "          \"$ref\": \"#/$defs/uid\""]
#[doc = "        },"]
#[doc = "        \"unit_convention\": {"]
#[doc = "          \"description\": \"units used for physical network parameters\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"NATURAL_UNITS\","]
#[doc = "            \"PER_UNIT_COMPONENT_BASE\","]
#[doc = "            \"PER_UNIT_SYSTEM_BASE\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"hvdc_p2p\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"structure to hold point-to-point hvdc line data\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"bus_fr\","]
#[doc = "          \"bus_to\","]
#[doc = "          \"status\","]
#[doc = "          \"uid\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"base_kv_dc\": {"]
#[doc = "            \"description\": \"[kV] base voltage at the dc side\","]
#[doc = "            \"$ref\": \"#/$defs/positive_number\""]
#[doc = "          },"]
#[doc = "          \"bus_fr\": {"]
#[doc = "            \"description\": \"uid of bus at the from terminal of hvdc line\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"bus_to\": {"]
#[doc = "            \"description\": \"uid of bus at the to terminal of hvdc line\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"cm_ub_fr\": {"]
#[doc = "            \"description\": \"[kA or pu] ac persistent current rating, from terminal (if in pu, use from bus base_kv)\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"cm_ub_to\": {"]
#[doc = "            \"description\": \"[kA or pu] ac persistent current rating, to terminal (if in pu, use to bus base_kv)\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional hvdc point-to-point parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"loss_a\": {"]
#[doc = "            \"description\": \"[MW or pu] standby loss\","]
#[doc = "            \"default\": 0,"]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"loss_b\": {"]
#[doc = "            \"description\": \"[kV or pu] loss proportional to current magnitude (if in pu, base voltage corresponds to base_kv_dc)\","]
#[doc = "            \"default\": 0,"]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"loss_c\": {"]
#[doc = "            \"description\": \"[Ohm or pu] loss proportional to current magnitude squared (if in pu, base voltage corresponds to base_kv_dc)\","]
#[doc = "            \"default\": 0,"]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"HVDC line name\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"nominal_mva\": {"]
#[doc = "            \"description\": \"[MVA] nominal apparent power of hvdc line\","]
#[doc = "            \"$ref\": \"#/$defs/positive_number\""]
#[doc = "          },"]
#[doc = "          \"p\": {"]
#[doc = "            \"description\": \"number of poles; 1 => monopole, 2 => bipole\","]
#[doc = "            \"type\": \"integer\","]
#[doc = "            \"maximum\": 2.0,"]
#[doc = "            \"minimum\": 1.0"]
#[doc = "          },"]
#[doc = "          \"pdc_fr_lb\": {"]
#[doc = "            \"description\": \"[MW or pu] minimum active power entering hvdc line at from bus\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"pdc_fr_ub\": {"]
#[doc = "            \"description\": \"[MW or pu] maximum active power entering hvdc line at from bus\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"pdc_to_lb\": {"]
#[doc = "            \"description\": \"[MW or pu] minimum active power entering hvdc line at to bus\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"pdc_to_ub\": {"]
#[doc = "            \"description\": \"[MW or pu] maximum active power entering hvdc line at to bus\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"persistent_outage_duration\": {"]
#[doc = "            \"description\": \"[hours] expected duration of persistent outage (time between outage and crews re-energizing the branch)\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"persistent_outage_rate\": {"]
#[doc = "            \"description\": \"[events/year] number of expected persistent outages per year (outages not cleared by reconnectors)\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"phi_lb\": {"]
#[doc = "            \"description\": \"[deg] only meaningful if technology == LCC; firing angle minimum\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"phi_ub\": {"]
#[doc = "            \"description\": \"[deg] only meaningful if technology == LCC; firing angle maximum\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"qdc_fr_lb\": {"]
#[doc = "            \"description\": \"[MVAr or pu] minimum reactive power entering hvdc line at from bus\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"qdc_fr_ub\": {"]
#[doc = "            \"description\": \"[MVAr or pu] maximum reactive power entering hvdc line at from bus\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"qdc_to_lb\": {"]
#[doc = "            \"description\": \"[MVAr or pu] minimum reactive power entering hvdc line at to bus\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"qdc_to_ub\": {"]
#[doc = "            \"description\": \"[MW or pu] maximum active power entering hvdc line at to bus\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"r\": {"]
#[doc = "            \"description\": \"[Ohm or pu] dc line resistance (if in pu, base voltage corresponds to base_kv_dc)\","]
#[doc = "            \"default\": 0.0,"]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"sm_ub\": {"]
#[doc = "            \"description\": \"[MVA or pu] ac persistent apparent power rating\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"status\": {"]
#[doc = "            \"$ref\": \"#/$defs/status\""]
#[doc = "          },"]
#[doc = "          \"technology\": {"]
#[doc = "            \"description\": \"power conversion technology\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"LCC\","]
#[doc = "              \"VSC\","]
#[doc = "              \"MMC\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"transient_outage_rate\": {"]
#[doc = "            \"description\": \"[events/year] number of expected transient outages per year (outages cleared by reconnectors or other)\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"vm_dc_lb\": {"]
#[doc = "            \"description\": \"[kV or pu] minimum voltage at the dc side\","]
#[doc = "            \"$ref\": \"#/$defs/positive_number\""]
#[doc = "          },"]
#[doc = "          \"vm_dc_ub\": {"]
#[doc = "            \"description\": \"[kV or pu] maximum voltage at the dc side\","]
#[doc = "            \"$ref\": \"#/$defs/positive_number\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"load\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"structure to hold load (consumer) data using ZIP model\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"bus\","]
#[doc = "          \"pd\","]
#[doc = "          \"qd\","]
#[doc = "          \"status\","]
#[doc = "          \"uid\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"bus\": {"]
#[doc = "            \"description\": \"uid of bus to which load is connected to\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional bus parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"load name\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"nominal_mva\": {"]
#[doc = "            \"description\": \"[MVA] nominal power of load\","]
#[doc = "            \"$ref\": \"#/$defs/positive_number\""]
#[doc = "          },"]
#[doc = "          \"pd\": {"]
#[doc = "            \"description\": \"active power demand\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"pd_i\": {"]
#[doc = "            \"description\": \"constant current active power demand at v_bus = 1.0 pu\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"pd_y\": {"]
#[doc = "            \"description\": \"constant impedance active power demand at v_bus = 1.0 pu\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"qd\": {"]
#[doc = "            \"description\": \"reactive power demand\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"qd_i\": {"]
#[doc = "            \"description\": \"constant current reactive power demand at v_bus = 1.0 pu\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"qd_y\": {"]
#[doc = "            \"description\": \"constant impedance reactive power demand at v_bus = 1.0 pu\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"status\": {"]
#[doc = "            \"$ref\": \"#/$defs/status\""]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"reserve\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"structure to hold reserve product and requirement data\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"reserve_type\","]
#[doc = "          \"status\","]
#[doc = "          \"uid\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional reserve parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"name of reserve product\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"participants\": {"]
#[doc = "            \"description\": \"uid of generators contributing to this reserve\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"$ref\": \"#/$defs/uid\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"pg_down\": {"]
#[doc = "            \"description\": \"[MW or pu] downward active power required by this reserve\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"pg_up\": {"]
#[doc = "            \"description\": \"[MW or pu] upward active power required by this reserve\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"reserve_type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"PRIMARY\","]
#[doc = "              \"SECONDARY\","]
#[doc = "              \"TERTIARY\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"status\": {"]
#[doc = "            \"$ref\": \"#/$defs/status\""]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"shunt\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"structure to hold shunt data\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"bs\","]
#[doc = "          \"bus\","]
#[doc = "          \"gs\","]
#[doc = "          \"num_steps_ub\","]
#[doc = "          \"status\","]
#[doc = "          \"uid\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"bs\": {"]
#[doc = "            \"description\": \"[MVAr or pu] reactive power demand at v_bus = 1.0 pu, per step of each shunt section\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"type\": \"number\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"bus\": {"]
#[doc = "            \"description\": \"uid of bus to which shunt is connected to\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional shunt parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"gs\": {"]
#[doc = "            \"description\": \"[MW or pu] active power demand at v_bus = 1.0 pu, per step of each shunt section\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"shunt name\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"nominal_mva\": {"]
#[doc = "            \"description\": \"[MVA] nominal apparent power of shunt (nameplate capacity)\","]
#[doc = "            \"$ref\": \"#/$defs/positive_number\""]
#[doc = "          },"]
#[doc = "          \"num_steps_ub\": {"]
#[doc = "            \"description\": \"upper bound for number of energized steps of shunt section (lower bound is always 0)\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_integer\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"$ref\": \"#/$defs/nonnegative_integer\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"status\": {"]
#[doc = "            \"$ref\": \"#/$defs/status\""]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"storage\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"structure to hold storage (battery) data\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"bus\","]
#[doc = "          \"charge_efficiency\","]
#[doc = "          \"discharge_efficiency\","]
#[doc = "          \"ps_ex\","]
#[doc = "          \"qs_ex\","]
#[doc = "          \"status\","]
#[doc = "          \"uid\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"bus\": {"]
#[doc = "            \"description\": \"uid of bus to which generator is connected to\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"charge_efficiency\": {"]
#[doc = "            \"description\": \"[-] charge efficiency, in (0, 1]\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"maximum\": 1.0,"]
#[doc = "                \"exclusiveMinimum\": 0.0"]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"charge_ub\": {"]
#[doc = "            \"description\": \"[MW or pu] maximum rate of charge\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"cm_ub\": {"]
#[doc = "            \"description\": \"[kA or pu] converter current output rating\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"discharge_efficiency\": {"]
#[doc = "            \"description\": \"[-] discharge efficiency, in (0, 1]\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"maximum\": 1.0,"]
#[doc = "                \"exclusiveMinimum\": 0.0"]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"discharge_ub\": {"]
#[doc = "            \"description\": \"[MW or pu] maximum rate of discharge\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"energy_ub\": {"]
#[doc = "            \"description\": \"[MWh or pu*h] maximum state of charge\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional storage parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"storage name\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"nominal_mva\": {"]
#[doc = "            \"description\": \"[MVA] nominal apparent power of storage (nameplate capacity)\","]
#[doc = "            \"$ref\": \"#/$defs/positive_number\""]
#[doc = "          },"]
#[doc = "          \"ps_delta_lb\": {"]
#[doc = "            \"description\": \"[MW/h or pu/h] maximum active power decrease per hour\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"ps_delta_ub\": {"]
#[doc = "            \"description\": \"[MW/h or pu/h] maximum active power increase per hour\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"ps_ex\": {"]
#[doc = "            \"description\": \"converter standby active power exogenous draw\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"qs_ex\": {"]
#[doc = "            \"description\": \"converter standby reactive power exogenous draw\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"qs_lb\": {"]
#[doc = "            \"description\": \"[MVAr or pu] minumum reactive power injection\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"qs_ub\": {"]
#[doc = "            \"description\": \"[MVAr or pu] maximum reactive power injection\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"sm_ub\": {"]
#[doc = "            \"description\": \"[MVA or pu] converter apparent power rating\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"status\": {"]
#[doc = "            \"$ref\": \"#/$defs/status\""]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"switch\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"bus_fr\","]
#[doc = "          \"bus_to\","]
#[doc = "          \"status\","]
#[doc = "          \"uid\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"bus_fr\": {"]
#[doc = "            \"description\": \"uid of bus at the from terminal of switch\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"bus_to\": {"]
#[doc = "            \"description\": \"uid of bus at the to terminal of switch\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"cm_ub\": {"]
#[doc = "            \"description\": \"[kA or pu] current limit\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional switch parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"name of switch\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"nominal_mva\": {"]
#[doc = "            \"description\": \"[MVA] nominal apparent power of switch (nameplate capacity)\","]
#[doc = "            \"$ref\": \"#/$defs/positive_number\""]
#[doc = "          },"]
#[doc = "          \"sm_ub\": {"]
#[doc = "            \"description\": \"[MVA or pu] apparent power flow limit\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"status\": {"]
#[doc = "            \"$ref\": \"#/$defs/status\""]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"transformer\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"structure to hold 2-winding transformer and phase shifter data using simplified (4-parameter circuit) model\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"b\","]
#[doc = "          \"bus_fr\","]
#[doc = "          \"bus_to\","]
#[doc = "          \"g\","]
#[doc = "          \"r\","]
#[doc = "          \"status\","]
#[doc = "          \"uid\","]
#[doc = "          \"x\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"b\": {"]
#[doc = "            \"description\": \"[S or pu] shunt susceptance of transformer at from terminal (magnetizing branch)\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"bus_fr\": {"]
#[doc = "            \"description\": \"uid of bus at the from terminal of transformer\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"bus_to\": {"]
#[doc = "            \"description\": \"uid of bus at the to terminal of transformer\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"cm_ub_a\": {"]
#[doc = "            \"description\": \"[kA or pu] persistent current rating, referred to from side\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"cm_ub_b\": {"]
#[doc = "            \"description\": \"[kA or pu] 4-hour current rating, referred to from side\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"cm_ub_c\": {"]
#[doc = "            \"description\": \"[kA or pu] 15-minute current rating, referred to from side\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional transformer parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"g\": {"]
#[doc = "            \"description\": \"[S or pu] shunt conductance of transformer at from terminal (magnetizing branch)\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"transformer name\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"nominal_mva\": {"]
#[doc = "            \"description\": \"[MVA] nominal apparent power of transformer\","]
#[doc = "            \"$ref\": \"#/$defs/positive_number\""]
#[doc = "          },"]
#[doc = "          \"persistent_outage_duration\": {"]
#[doc = "            \"description\": \"[hours] expected duration of persistent outage (time between outage and crews re-energizing the branch)\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"persistent_outage_rate\": {"]
#[doc = "            \"description\": \"[events/year] number of expected persistent outages per year (outages not cleared by reconnectors)\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"r\": {"]
#[doc = "            \"description\": \"[Ohm or pu] series resistance of line\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"sm_ub_a\": {"]
#[doc = "            \"description\": \"[MVA or pu] persistent apparent power rating, referred to from side\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"sm_ub_b\": {"]
#[doc = "            \"description\": \"[MVA or pu] 4-hour apparent power rating, referred to from side\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"sm_ub_c\": {"]
#[doc = "            \"description\": \"[MVA or pu] 15-minute apparent power rating, referred to from side\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"status\": {"]
#[doc = "            \"$ref\": \"#/$defs/status\""]
#[doc = "          },"]
#[doc = "          \"ta_lb\": {"]
#[doc = "            \"description\": \"[deg] minimum angle phase shift (angle difference = va_from - va_to - angle_shift)\","]
#[doc = "            \"default\": 0,"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"ta_steps\": {"]
#[doc = "            \"description\": \"number of discrete steps between ta_lb and ta_ub (including limit values)\","]
#[doc = "            \"default\": 1,"]
#[doc = "            \"$ref\": \"#/$defs/positive_integer\""]
#[doc = "          },"]
#[doc = "          \"ta_ub\": {"]
#[doc = "            \"description\": \"[deg] maximum angle phase shift (angle difference = va_from - va_to - angle_shift)\","]
#[doc = "            \"default\": 0,"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"tm_lb\": {"]
#[doc = "            \"description\": \"[-] minimum tap ratio (1.0 correspond to nominal ratio, inner_vm_from = vm_from * tap_value)\","]
#[doc = "            \"default\": 1.0,"]
#[doc = "            \"$ref\": \"#/$defs/positive_number\""]
#[doc = "          },"]
#[doc = "          \"tm_steps\": {"]
#[doc = "            \"description\": \"number of discrete steps between tm_lb and tm_ub (including limit values)\","]
#[doc = "            \"default\": 1,"]
#[doc = "            \"$ref\": \"#/$defs/positive_integer\""]
#[doc = "          },"]
#[doc = "          \"tm_ub\": {"]
#[doc = "            \"description\": \"[-] maximum tap ratio (1.0 correspond to nominal ratio, inner_vm_from = vm_from * tap_value)\","]
#[doc = "            \"default\": 1.0,"]
#[doc = "            \"$ref\": \"#/$defs/positive_number\""]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"x\": {"]
#[doc = "            \"description\": \"[Ohm or pu] series impedance of line\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"zone\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"geographical subset of the electrical network commonly associated with market purposes (e.g., define sub-markets within a large interconnected system, defining different areas for reserve products, etc.)\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"status\","]
#[doc = "          \"uid\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional zone parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"zone name\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"status\": {"]
#[doc = "            \"description\": \"binary indicator of whether zone should be included or omitted (if omitted all elements within zone should be omitted); 1=>included, 0=>omitted\","]
#[doc = "            \"$ref\": \"#/$defs/status\""]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataNetwork {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub ac_line: ::std::vec::Vec<CtmDataNetworkAcLineItem>,
    pub area: ::std::vec::Vec<CtmDataNetworkAreaItem>,
    pub bus: ::std::vec::Vec<CtmDataNetworkBusItem>,
    pub gen: ::std::vec::Vec<CtmDataNetworkGenItem>,
    pub global_params: CtmDataNetworkGlobalParams,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub hvdc_p2p: ::std::vec::Vec<CtmDataNetworkHvdcP2pItem>,
    pub load: ::std::vec::Vec<CtmDataNetworkLoadItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub reserve: ::std::vec::Vec<CtmDataNetworkReserveItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub shunt: ::std::vec::Vec<CtmDataNetworkShuntItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub storage: ::std::vec::Vec<CtmDataNetworkStorageItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub switch: ::std::vec::Vec<CtmDataNetworkSwitchItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub transformer: ::std::vec::Vec<CtmDataNetworkTransformerItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub zone: ::std::vec::Vec<CtmDataNetworkZoneItem>,
}
impl ::std::convert::From<&CtmDataNetwork> for CtmDataNetwork {
    fn from(value: &CtmDataNetwork) -> Self {
        value.clone()
    }
}
impl CtmDataNetwork {
    pub fn builder() -> builder::CtmDataNetwork {
        Default::default()
    }
}
#[doc = "structure to hold ac line data using concentrated (6-parameter circuit) PI model"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold ac line data using concentrated (6-parameter circuit) PI model\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bus_fr\","]
#[doc = "    \"bus_to\","]
#[doc = "    \"r\","]
#[doc = "    \"status\","]
#[doc = "    \"uid\","]
#[doc = "    \"x\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"b_fr\": {"]
#[doc = "      \"description\": \"[S or pu] shunt susceptance of line at from terminal\","]
#[doc = "      \"default\": 0,"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"b_to\": {"]
#[doc = "      \"description\": \"[S or pu] shunt susceptance of line at to terminal\","]
#[doc = "      \"default\": 0,"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"bus_fr\": {"]
#[doc = "      \"description\": \"uid of bus at the from terminal of ac line\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"bus_to\": {"]
#[doc = "      \"description\": \"uid of bus at the to terminal of ac line\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"cm_ub_a\": {"]
#[doc = "      \"description\": \"[kA or pu] persistent current rating\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cm_ub_b\": {"]
#[doc = "      \"description\": \"[kA or pu] 4-hour current rating\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cm_ub_c\": {"]
#[doc = "      \"description\": \"[kA or pu] 15-minute current rating\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional ac line parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"g_fr\": {"]
#[doc = "      \"description\": \"[S or pu] shunt conductance of line at from terminal\","]
#[doc = "      \"default\": 0,"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"g_to\": {"]
#[doc = "      \"description\": \"[S or pu] shunt conductance of line at to terminal\","]
#[doc = "      \"default\": 0,"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"line name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nominal_mva\": {"]
#[doc = "      \"description\": \"[MVA] nominal apparent power of ac line\","]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    \"persistent_outage_duration\": {"]
#[doc = "      \"description\": \"[hours] expected duration of persistent outage (time between outage and crews re-energizing the branch)\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"persistent_outage_rate\": {"]
#[doc = "      \"description\": \"[events/year] number of expected persistent outages per year (outages not cleared by reconnectors)\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"r\": {"]
#[doc = "      \"description\": \"[Ohm or pu] series resistance of line\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"sm_ub_a\": {"]
#[doc = "      \"description\": \"[MVA or pu] persistent apparent power rating\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sm_ub_b\": {"]
#[doc = "      \"description\": \"[MVA or pu] 4-hour apparent power rating\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sm_ub_c\": {"]
#[doc = "      \"description\": \"[MVA or pu] 15-minute apparent power rating\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/$defs/status\""]
#[doc = "    },"]
#[doc = "    \"transient_outage_rate\": {"]
#[doc = "      \"description\": \"[events/year] number of expected transient outages per year (outages cleared by reconnectors)\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"vad_lb\": {"]
#[doc = "      \"description\": \"[deg] voltage angle difference lower bound (stability)\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"vad_ub\": {"]
#[doc = "      \"description\": \"[deg] voltage angle difference upper bound (stability)\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"x\": {"]
#[doc = "      \"description\": \"[Ohm or pu] series impedance of line\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataNetworkAcLineItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub b_fr: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub b_to: ::std::option::Option<f64>,
    #[doc = "uid of bus at the from terminal of ac line"]
    pub bus_fr: Uid,
    #[doc = "uid of bus at the to terminal of ac line"]
    pub bus_to: Uid,
    #[doc = "[kA or pu] persistent current rating"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cm_ub_a: ::std::option::Option<CtmDataNetworkAcLineItemCmUbA>,
    #[doc = "[kA or pu] 4-hour current rating"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cm_ub_b: ::std::option::Option<CtmDataNetworkAcLineItemCmUbB>,
    #[doc = "[kA or pu] 15-minute current rating"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cm_ub_c: ::std::option::Option<CtmDataNetworkAcLineItemCmUbC>,
    #[doc = "additional ac line parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub g_fr: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub g_to: ::std::option::Option<f64>,
    #[doc = "line name"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "[MVA] nominal apparent power of ac line"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nominal_mva: ::std::option::Option<PositiveNumber>,
    #[doc = "[hours] expected duration of persistent outage (time between outage and crews re-energizing the branch)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub persistent_outage_duration: ::std::option::Option<NonnegativeNumber>,
    #[doc = "[events/year] number of expected persistent outages per year (outages not cleared by reconnectors)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub persistent_outage_rate: ::std::option::Option<NonnegativeNumber>,
    pub r: f64,
    #[doc = "[MVA or pu] persistent apparent power rating"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sm_ub_a: ::std::option::Option<CtmDataNetworkAcLineItemSmUbA>,
    #[doc = "[MVA or pu] 4-hour apparent power rating"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sm_ub_b: ::std::option::Option<CtmDataNetworkAcLineItemSmUbB>,
    #[doc = "[MVA or pu] 15-minute apparent power rating"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sm_ub_c: ::std::option::Option<CtmDataNetworkAcLineItemSmUbC>,
    pub status: Status,
    #[doc = "[events/year] number of expected transient outages per year (outages cleared by reconnectors)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub transient_outage_rate: ::std::option::Option<NonnegativeNumber>,
    pub uid: Uid,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub vad_lb: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub vad_ub: ::std::option::Option<f64>,
    pub x: f64,
}
impl ::std::convert::From<&CtmDataNetworkAcLineItem> for CtmDataNetworkAcLineItem {
    fn from(value: &CtmDataNetworkAcLineItem) -> Self {
        value.clone()
    }
}
impl CtmDataNetworkAcLineItem {
    pub fn builder() -> builder::CtmDataNetworkAcLineItem {
        Default::default()
    }
}
#[doc = "[kA or pu] persistent current rating"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[kA or pu] persistent current rating\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkAcLineItemCmUbA {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkAcLineItemCmUbA {
    fn from(value: &CtmDataNetworkAcLineItemCmUbA) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkAcLineItemCmUbA {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkAcLineItemCmUbA {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "[kA or pu] 4-hour current rating"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[kA or pu] 4-hour current rating\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkAcLineItemCmUbB {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkAcLineItemCmUbB {
    fn from(value: &CtmDataNetworkAcLineItemCmUbB) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkAcLineItemCmUbB {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkAcLineItemCmUbB {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "[kA or pu] 15-minute current rating"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[kA or pu] 15-minute current rating\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkAcLineItemCmUbC {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkAcLineItemCmUbC {
    fn from(value: &CtmDataNetworkAcLineItemCmUbC) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkAcLineItemCmUbC {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkAcLineItemCmUbC {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "[MVA or pu] persistent apparent power rating"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MVA or pu] persistent apparent power rating\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkAcLineItemSmUbA {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkAcLineItemSmUbA {
    fn from(value: &CtmDataNetworkAcLineItemSmUbA) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkAcLineItemSmUbA {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkAcLineItemSmUbA {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "[MVA or pu] 4-hour apparent power rating"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MVA or pu] 4-hour apparent power rating\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkAcLineItemSmUbB {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkAcLineItemSmUbB {
    fn from(value: &CtmDataNetworkAcLineItemSmUbB) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkAcLineItemSmUbB {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkAcLineItemSmUbB {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "[MVA or pu] 15-minute apparent power rating"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MVA or pu] 15-minute apparent power rating\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkAcLineItemSmUbC {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkAcLineItemSmUbC {
    fn from(value: &CtmDataNetworkAcLineItemSmUbC) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkAcLineItemSmUbC {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkAcLineItemSmUbC {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "geographical subset of the electrical network with common Automatic Generation Control (AGC) and responsible for its Area Control Error (ACE)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"geographical subset of the electrical network with common Automatic Generation Control (AGC) and responsible for its Area Control Error (ACE)\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"status\","]
#[doc = "    \"uid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional area parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"area name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"binary indicator of whether area should be included or omitted (if omitted all elements within area should be omitted); 1=>included, 0=>omitted\","]
#[doc = "      \"$ref\": \"#/$defs/status\""]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataNetworkAreaItem {
    #[doc = "additional area parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    #[doc = "area name"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "binary indicator of whether area should be included or omitted (if omitted all elements within area should be omitted); 1=>included, 0=>omitted"]
    pub status: Status,
    pub uid: Uid,
}
impl ::std::convert::From<&CtmDataNetworkAreaItem> for CtmDataNetworkAreaItem {
    fn from(value: &CtmDataNetworkAreaItem) -> Self {
        value.clone()
    }
}
impl CtmDataNetworkAreaItem {
    pub fn builder() -> builder::CtmDataNetworkAreaItem {
        Default::default()
    }
}
#[doc = "structure to hold bus data"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold bus data\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"base_kv\","]
#[doc = "    \"status\","]
#[doc = "    \"uid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"area\": {"]
#[doc = "      \"description\": \"uid for area to which bus belongs to\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"base_kv\": {"]
#[doc = "      \"description\": \"bus base (nominal) voltage\","]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional bus parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"bus name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/$defs/status\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"bus type for power flow calculations (PV, PQ, or slack)\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"PQ\","]
#[doc = "            \"PV\","]
#[doc = "            \"slack\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"vm_lb\": {"]
#[doc = "      \"description\": \"bus voltage lower bound\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/positive_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"vm_ub\": {"]
#[doc = "      \"description\": \"bus voltage upper bound\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/positive_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"zone\": {"]
#[doc = "      \"description\": \"uid for zone to which bus belongs to\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataNetworkBusItem {
    #[doc = "uid for area to which bus belongs to"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub area: ::std::option::Option<Uid>,
    #[doc = "bus base (nominal) voltage"]
    pub base_kv: PositiveNumber,
    #[doc = "additional bus parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    #[doc = "bus name"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    pub status: Status,
    #[doc = "bus type for power flow calculations (PV, PQ, or slack)"]
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<CtmDataNetworkBusItemType>,
    pub uid: Uid,
    #[doc = "bus voltage lower bound"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub vm_lb: ::std::option::Option<CtmDataNetworkBusItemVmLb>,
    #[doc = "bus voltage upper bound"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub vm_ub: ::std::option::Option<CtmDataNetworkBusItemVmUb>,
    #[doc = "uid for zone to which bus belongs to"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub zone: ::std::option::Option<Uid>,
}
impl ::std::convert::From<&CtmDataNetworkBusItem> for CtmDataNetworkBusItem {
    fn from(value: &CtmDataNetworkBusItem) -> Self {
        value.clone()
    }
}
impl CtmDataNetworkBusItem {
    pub fn builder() -> builder::CtmDataNetworkBusItem {
        Default::default()
    }
}
#[doc = "bus type for power flow calculations (PV, PQ, or slack)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"bus type for power flow calculations (PV, PQ, or slack)\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"PQ\","]
#[doc = "        \"PV\","]
#[doc = "        \"slack\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkBusItemType {
    Variant0(CtmDataNetworkBusItemTypeVariant0),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkBusItemType {
    fn from(value: &CtmDataNetworkBusItemType) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<CtmDataNetworkBusItemTypeVariant0> for CtmDataNetworkBusItemType {
    fn from(value: CtmDataNetworkBusItemTypeVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkBusItemType {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "`CtmDataNetworkBusItemTypeVariant0`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"PQ\","]
#[doc = "    \"PV\","]
#[doc = "    \"slack\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CtmDataNetworkBusItemTypeVariant0 {
    #[serde(rename = "PQ")]
    Pq,
    #[serde(rename = "PV")]
    Pv,
    #[serde(rename = "slack")]
    Slack,
}
impl ::std::convert::From<&Self> for CtmDataNetworkBusItemTypeVariant0 {
    fn from(value: &CtmDataNetworkBusItemTypeVariant0) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CtmDataNetworkBusItemTypeVariant0 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pq => write!(f, "PQ"),
            Self::Pv => write!(f, "PV"),
            Self::Slack => write!(f, "slack"),
        }
    }
}
impl ::std::str::FromStr for CtmDataNetworkBusItemTypeVariant0 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "PQ" => Ok(Self::Pq),
            "PV" => Ok(Self::Pv),
            "slack" => Ok(Self::Slack),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CtmDataNetworkBusItemTypeVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CtmDataNetworkBusItemTypeVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CtmDataNetworkBusItemTypeVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "bus voltage lower bound"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"bus voltage lower bound\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkBusItemVmLb {
    PositiveNumber(PositiveNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkBusItemVmLb {
    fn from(value: &CtmDataNetworkBusItemVmLb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<PositiveNumber> for CtmDataNetworkBusItemVmLb {
    fn from(value: PositiveNumber) -> Self {
        Self::PositiveNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkBusItemVmLb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "bus voltage upper bound"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"bus voltage upper bound\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkBusItemVmUb {
    PositiveNumber(PositiveNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkBusItemVmUb {
    fn from(value: &CtmDataNetworkBusItemVmUb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<PositiveNumber> for CtmDataNetworkBusItemVmUb {
    fn from(value: PositiveNumber) -> Self {
        Self::PositiveNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkBusItemVmUb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "structure to hold generator data"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold generator data\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bus\","]
#[doc = "    \"status\","]
#[doc = "    \"uid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bus\": {"]
#[doc = "      \"description\": \"uid of bus to which generator is connected to\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"cost_pg_model\": {"]
#[doc = "      \"description\": \"type of generation cost model (i.e., function translating power/energy to money); POLYNOMIAL => cost_pg_parameters is an array with n+1 coefficients <a_i> for f(x) = a_0 + a_1 x^1 + ... + a_n x^n; PIECEWISE_LINEAR => cost_pg_parameters is a series of values <x_i, f_i> and cost (f) should be interpolated linearly in between points; MARGINAL_COST => cost_pg_parameters is a series of values <b_i, m_i>, where m_i is a marginal cost ($/MWh or $/(pu*h)) and b_i is the amoung of power (MWh or pu*h) sold at marginal cost m_i\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"POLYNOMIAL\","]
#[doc = "        \"PIECEWISE_LINEAR\","]
#[doc = "        \"MARGINAL_COST\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cost_pg_parameters\": {"]
#[doc = "      \"description\": \"parameters of generation cost function, can be time dependent\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"number\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/xy_pairs\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"down_time_lb\": {"]
#[doc = "      \"description\": \"[h] minimim time the unit can be out of service (a.k.a., minimum down time)\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional gen parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"forced_outage_rate\": {"]
#[doc = "      \"description\": \"[-] fraction of time the generator is out of service because of forced outages (i.e., hours out of service---because of failures---during a year, divided by 8760)\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minumum\": 0"]
#[doc = "    },"]
#[doc = "    \"in_service_time_lb\": {"]
#[doc = "      \"description\": \"[h] minimim time the unit can be in service (a.k.a., minimum up time)\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"in_service_time_ub\": {"]
#[doc = "      \"description\": \"[h] maximum time the unit can be in service (commitment == 1)\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"mean_time_to_failure\": {"]
#[doc = "      \"description\": \"[h] mean time to occurence of a failure; failures can be assumed to follow a Poisson process\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"mean_time_to_repair\": {"]
#[doc = "      \"description\": \"[h] mean time to repair a failure\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"generator name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nominal_mva\": {"]
#[doc = "      \"description\": \"[MVA] nominal apparent power of generator (nameplate capacity)\","]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    \"pg_delta_lb\": {"]
#[doc = "      \"description\": \"[MW/h or pu/h] maximum active power decrease per hour\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"pg_delta_ub\": {"]
#[doc = "      \"description\": \"[MW/h or pu/h] maximum active power increase per hour\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"pg_lb\": {"]
#[doc = "      \"description\": \"[MW or pu] lower bound of active power injection (rectangular operating zone)\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pg_ub\": {"]
#[doc = "      \"description\": \"[MW or pu] upper bound of active power injection (rectangular operating zone)\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"primary_source\": {"]
#[doc = "      \"description\": \"primary energy source\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"COAL\","]
#[doc = "        \"OIL\","]
#[doc = "        \"GAS\","]
#[doc = "        \"NUCLEAR\","]
#[doc = "        \"BIOMASS\","]
#[doc = "        \"GEOTHERMAL\","]
#[doc = "        \"SOLAR\","]
#[doc = "        \"WIND\","]
#[doc = "        \"HYDRO\","]
#[doc = "        \"OTHER\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"primary_source_subtype\": {"]
#[doc = "      \"description\": \"subtype of primary energy source; thermal classification taken from https://www.eia.gov/survey/form/eia_923/instructions.pdf\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"ANTRHC_BITMN_COAL\","]
#[doc = "        \"WASTE_COAL\","]
#[doc = "        \"DISTILLATE_FUEL_OIL\","]
#[doc = "        \"WASTE_OIL\","]
#[doc = "        \"PETROLEUM_COKE\","]
#[doc = "        \"RESIDUAL_FUEL_OIL\","]
#[doc = "        \"NATURAL_GAS\","]
#[doc = "        \"OTHER_GAS\","]
#[doc = "        \"NUCLEAR\","]
#[doc = "        \"AG_BIPRODUCT\","]
#[doc = "        \"MUNICIPAL_WASTE\","]
#[doc = "        \"WOOD_WASTE\","]
#[doc = "        \"GEOTHERMAL\","]
#[doc = "        \"SOLAR_PV\","]
#[doc = "        \"SOLAR_CSP\","]
#[doc = "        \"WIND_ONSHORE\","]
#[doc = "        \"WIND_OFFSHORE\","]
#[doc = "        \"HYDRO_RUN_OF_THE_RIVER\","]
#[doc = "        \"HYDRO_DAM\","]
#[doc = "        \"HYDRO_PUMPED_STORAGE\","]
#[doc = "        \"OTHER\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"qg_lb\": {"]
#[doc = "      \"description\": \"[MVAr or pu] lower bound of reactive power injection (rectangular operating zone)\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"qg_ub\": {"]
#[doc = "      \"description\": \"[MVAr or pu] upper bound of reactive power injection (rectangular operating zone)\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"scheduled_maintenance_rate\": {"]
#[doc = "      \"description\": \"[-] fraction of time the generator is out of service because of scheduled maintenance (i.e., hours out of service---because of scheduled maintenance---during a year, divided by 8760)\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minumum\": 0"]
#[doc = "    },"]
#[doc = "    \"service_required\": {"]
#[doc = "      \"description\": \"whether generator must be in service (e.g., nuclear power plant) or out of service (e.g., generator during maintenance or after an outage); 0 => no requirement, 1 => fixed in service, 2 => fixed out of service\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"maximum\": 2.0,"]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"shutdown_cost\": {"]
#[doc = "      \"description\": \"[$] cost of shutting down the unit\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"startup_cost_cold\": {"]
#[doc = "      \"description\": \"[$] cost of starting the unit after being off > startup_time_warm hours\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"startup_cost_hot\": {"]
#[doc = "      \"description\": \"[$] cost of starting the unit after being off <= startup_time_hot hours\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"startup_cost_warm\": {"]
#[doc = "      \"description\": \"[$] cost of starting the unit after being off > startup_time_hot hours, but <= startup_time_warm hours\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"startup_time_hot\": {"]
#[doc = "      \"description\": \"[h] maximum time the unit can be off before a hot startup\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"startup_time_warm\": {"]
#[doc = "      \"description\": \"[h] maximum time the unit can be off before a warm startup\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/$defs/status\""]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"vm_setpoint\": {"]
#[doc = "      \"description\": \"[kV or pu] target voltage magnitude of the bus that this generator connects to\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/positive_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataNetworkGenItem {
    #[doc = "uid of bus to which generator is connected to"]
    pub bus: Uid,
    #[doc = "type of generation cost model (i.e., function translating power/energy to money); POLYNOMIAL => cost_pg_parameters is an array with n+1 coefficients <a_i> for f(x) = a_0 + a_1 x^1 + ... + a_n x^n; PIECEWISE_LINEAR => cost_pg_parameters is a series of values <x_i, f_i> and cost (f) should be interpolated linearly in between points; MARGINAL_COST => cost_pg_parameters is a series of values <b_i, m_i>, where m_i is a marginal cost ($/MWh or $/(pu*h)) and b_i is the amoung of power (MWh or pu*h) sold at marginal cost m_i"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cost_pg_model: ::std::option::Option<CtmDataNetworkGenItemCostPgModel>,
    #[doc = "parameters of generation cost function, can be time dependent"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cost_pg_parameters: ::std::option::Option<CtmDataNetworkGenItemCostPgParameters>,
    #[doc = "[h] minimim time the unit can be out of service (a.k.a., minimum down time)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub down_time_lb: ::std::option::Option<NonnegativeNumber>,
    #[doc = "additional gen parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub forced_outage_rate: ::std::option::Option<f64>,
    #[doc = "[h] minimim time the unit can be in service (a.k.a., minimum up time)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub in_service_time_lb: ::std::option::Option<NonnegativeNumber>,
    #[doc = "[h] maximum time the unit can be in service (commitment == 1)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub in_service_time_ub: ::std::option::Option<NonnegativeNumber>,
    #[doc = "[h] mean time to occurence of a failure; failures can be assumed to follow a Poisson process"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mean_time_to_failure: ::std::option::Option<NonnegativeNumber>,
    #[doc = "[h] mean time to repair a failure"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mean_time_to_repair: ::std::option::Option<NonnegativeNumber>,
    #[doc = "generator name"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "[MVA] nominal apparent power of generator (nameplate capacity)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nominal_mva: ::std::option::Option<PositiveNumber>,
    #[doc = "[MW/h or pu/h] maximum active power decrease per hour"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pg_delta_lb: ::std::option::Option<NonnegativeNumber>,
    #[doc = "[MW/h or pu/h] maximum active power increase per hour"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pg_delta_ub: ::std::option::Option<NonnegativeNumber>,
    #[doc = "[MW or pu] lower bound of active power injection (rectangular operating zone)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pg_lb: ::std::option::Option<CtmDataNetworkGenItemPgLb>,
    #[doc = "[MW or pu] upper bound of active power injection (rectangular operating zone)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pg_ub: ::std::option::Option<CtmDataNetworkGenItemPgUb>,
    #[doc = "primary energy source"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub primary_source: ::std::option::Option<CtmDataNetworkGenItemPrimarySource>,
    #[doc = "subtype of primary energy source; thermal classification taken from https://www.eia.gov/survey/form/eia_923/instructions.pdf"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub primary_source_subtype: ::std::option::Option<CtmDataNetworkGenItemPrimarySourceSubtype>,
    #[doc = "[MVAr or pu] lower bound of reactive power injection (rectangular operating zone)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub qg_lb: ::std::option::Option<CtmDataNetworkGenItemQgLb>,
    #[doc = "[MVAr or pu] upper bound of reactive power injection (rectangular operating zone)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub qg_ub: ::std::option::Option<CtmDataNetworkGenItemQgUb>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scheduled_maintenance_rate: ::std::option::Option<f64>,
    #[doc = "whether generator must be in service (e.g., nuclear power plant) or out of service (e.g., generator during maintenance or after an outage); 0 => no requirement, 1 => fixed in service, 2 => fixed out of service"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub service_required: ::std::option::Option<CtmDataNetworkGenItemServiceRequired>,
    #[doc = "[$] cost of shutting down the unit"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub shutdown_cost: ::std::option::Option<CtmDataNetworkGenItemShutdownCost>,
    #[doc = "[$] cost of starting the unit after being off > startup_time_warm hours"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub startup_cost_cold: ::std::option::Option<CtmDataNetworkGenItemStartupCostCold>,
    #[doc = "[$] cost of starting the unit after being off <= startup_time_hot hours"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub startup_cost_hot: ::std::option::Option<CtmDataNetworkGenItemStartupCostHot>,
    #[doc = "[$] cost of starting the unit after being off > startup_time_hot hours, but <= startup_time_warm hours"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub startup_cost_warm: ::std::option::Option<CtmDataNetworkGenItemStartupCostWarm>,
    #[doc = "[h] maximum time the unit can be off before a hot startup"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub startup_time_hot: ::std::option::Option<NonnegativeNumber>,
    #[doc = "[h] maximum time the unit can be off before a warm startup"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub startup_time_warm: ::std::option::Option<NonnegativeNumber>,
    pub status: Status,
    pub uid: Uid,
    #[doc = "[kV or pu] target voltage magnitude of the bus that this generator connects to"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub vm_setpoint: ::std::option::Option<CtmDataNetworkGenItemVmSetpoint>,
}
impl ::std::convert::From<&CtmDataNetworkGenItem> for CtmDataNetworkGenItem {
    fn from(value: &CtmDataNetworkGenItem) -> Self {
        value.clone()
    }
}
impl CtmDataNetworkGenItem {
    pub fn builder() -> builder::CtmDataNetworkGenItem {
        Default::default()
    }
}
#[doc = "type of generation cost model (i.e., function translating power/energy to money); POLYNOMIAL => cost_pg_parameters is an array with n+1 coefficients <a_i> for f(x) = a_0 + a_1 x^1 + ... + a_n x^n; PIECEWISE_LINEAR => cost_pg_parameters is a series of values <x_i, f_i> and cost (f) should be interpolated linearly in between points; MARGINAL_COST => cost_pg_parameters is a series of values <b_i, m_i>, where m_i is a marginal cost ($/MWh or $/(pu*h)) and b_i is the amoung of power (MWh or pu*h) sold at marginal cost m_i"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"type of generation cost model (i.e., function translating power/energy to money); POLYNOMIAL => cost_pg_parameters is an array with n+1 coefficients <a_i> for f(x) = a_0 + a_1 x^1 + ... + a_n x^n; PIECEWISE_LINEAR => cost_pg_parameters is a series of values <x_i, f_i> and cost (f) should be interpolated linearly in between points; MARGINAL_COST => cost_pg_parameters is a series of values <b_i, m_i>, where m_i is a marginal cost ($/MWh or $/(pu*h)) and b_i is the amoung of power (MWh or pu*h) sold at marginal cost m_i\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"POLYNOMIAL\","]
#[doc = "    \"PIECEWISE_LINEAR\","]
#[doc = "    \"MARGINAL_COST\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CtmDataNetworkGenItemCostPgModel {
    #[serde(rename = "POLYNOMIAL")]
    Polynomial,
    #[serde(rename = "PIECEWISE_LINEAR")]
    PiecewiseLinear,
    #[serde(rename = "MARGINAL_COST")]
    MarginalCost,
}
impl ::std::convert::From<&Self> for CtmDataNetworkGenItemCostPgModel {
    fn from(value: &CtmDataNetworkGenItemCostPgModel) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CtmDataNetworkGenItemCostPgModel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Polynomial => write!(f, "POLYNOMIAL"),
            Self::PiecewiseLinear => write!(f, "PIECEWISE_LINEAR"),
            Self::MarginalCost => write!(f, "MARGINAL_COST"),
        }
    }
}
impl ::std::str::FromStr for CtmDataNetworkGenItemCostPgModel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "POLYNOMIAL" => Ok(Self::Polynomial),
            "PIECEWISE_LINEAR" => Ok(Self::PiecewiseLinear),
            "MARGINAL_COST" => Ok(Self::MarginalCost),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CtmDataNetworkGenItemCostPgModel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CtmDataNetworkGenItemCostPgModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CtmDataNetworkGenItemCostPgModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "parameters of generation cost function, can be time dependent"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"parameters of generation cost function, can be time dependent\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/xy_pairs\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkGenItemCostPgParameters {
    Variant0(::std::vec::Vec<f64>),
    Variant1(XyPairs),
    Variant2(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkGenItemCostPgParameters {
    fn from(value: &CtmDataNetworkGenItemCostPgParameters) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<f64>> for CtmDataNetworkGenItemCostPgParameters {
    fn from(value: ::std::vec::Vec<f64>) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<XyPairs> for CtmDataNetworkGenItemCostPgParameters {
    fn from(value: XyPairs) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkGenItemCostPgParameters {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant2(value)
    }
}
#[doc = "[MW or pu] lower bound of active power injection (rectangular operating zone)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MW or pu] lower bound of active power injection (rectangular operating zone)\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkGenItemPgLb {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkGenItemPgLb {
    fn from(value: &CtmDataNetworkGenItemPgLb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkGenItemPgLb {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkGenItemPgLb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[MW or pu] upper bound of active power injection (rectangular operating zone)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MW or pu] upper bound of active power injection (rectangular operating zone)\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkGenItemPgUb {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkGenItemPgUb {
    fn from(value: &CtmDataNetworkGenItemPgUb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkGenItemPgUb {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkGenItemPgUb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "primary energy source"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"primary energy source\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"COAL\","]
#[doc = "    \"OIL\","]
#[doc = "    \"GAS\","]
#[doc = "    \"NUCLEAR\","]
#[doc = "    \"BIOMASS\","]
#[doc = "    \"GEOTHERMAL\","]
#[doc = "    \"SOLAR\","]
#[doc = "    \"WIND\","]
#[doc = "    \"HYDRO\","]
#[doc = "    \"OTHER\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CtmDataNetworkGenItemPrimarySource {
    #[serde(rename = "COAL")]
    Coal,
    #[serde(rename = "OIL")]
    Oil,
    #[serde(rename = "GAS")]
    Gas,
    #[serde(rename = "NUCLEAR")]
    Nuclear,
    #[serde(rename = "BIOMASS")]
    Biomass,
    #[serde(rename = "GEOTHERMAL")]
    Geothermal,
    #[serde(rename = "SOLAR")]
    Solar,
    #[serde(rename = "WIND")]
    Wind,
    #[serde(rename = "HYDRO")]
    Hydro,
    #[serde(rename = "OTHER")]
    Other,
}
impl ::std::convert::From<&Self> for CtmDataNetworkGenItemPrimarySource {
    fn from(value: &CtmDataNetworkGenItemPrimarySource) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CtmDataNetworkGenItemPrimarySource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Coal => write!(f, "COAL"),
            Self::Oil => write!(f, "OIL"),
            Self::Gas => write!(f, "GAS"),
            Self::Nuclear => write!(f, "NUCLEAR"),
            Self::Biomass => write!(f, "BIOMASS"),
            Self::Geothermal => write!(f, "GEOTHERMAL"),
            Self::Solar => write!(f, "SOLAR"),
            Self::Wind => write!(f, "WIND"),
            Self::Hydro => write!(f, "HYDRO"),
            Self::Other => write!(f, "OTHER"),
        }
    }
}
impl ::std::str::FromStr for CtmDataNetworkGenItemPrimarySource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "COAL" => Ok(Self::Coal),
            "OIL" => Ok(Self::Oil),
            "GAS" => Ok(Self::Gas),
            "NUCLEAR" => Ok(Self::Nuclear),
            "BIOMASS" => Ok(Self::Biomass),
            "GEOTHERMAL" => Ok(Self::Geothermal),
            "SOLAR" => Ok(Self::Solar),
            "WIND" => Ok(Self::Wind),
            "HYDRO" => Ok(Self::Hydro),
            "OTHER" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CtmDataNetworkGenItemPrimarySource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CtmDataNetworkGenItemPrimarySource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CtmDataNetworkGenItemPrimarySource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "subtype of primary energy source; thermal classification taken from https://www.eia.gov/survey/form/eia_923/instructions.pdf"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"subtype of primary energy source; thermal classification taken from https://www.eia.gov/survey/form/eia_923/instructions.pdf\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"ANTRHC_BITMN_COAL\","]
#[doc = "    \"WASTE_COAL\","]
#[doc = "    \"DISTILLATE_FUEL_OIL\","]
#[doc = "    \"WASTE_OIL\","]
#[doc = "    \"PETROLEUM_COKE\","]
#[doc = "    \"RESIDUAL_FUEL_OIL\","]
#[doc = "    \"NATURAL_GAS\","]
#[doc = "    \"OTHER_GAS\","]
#[doc = "    \"NUCLEAR\","]
#[doc = "    \"AG_BIPRODUCT\","]
#[doc = "    \"MUNICIPAL_WASTE\","]
#[doc = "    \"WOOD_WASTE\","]
#[doc = "    \"GEOTHERMAL\","]
#[doc = "    \"SOLAR_PV\","]
#[doc = "    \"SOLAR_CSP\","]
#[doc = "    \"WIND_ONSHORE\","]
#[doc = "    \"WIND_OFFSHORE\","]
#[doc = "    \"HYDRO_RUN_OF_THE_RIVER\","]
#[doc = "    \"HYDRO_DAM\","]
#[doc = "    \"HYDRO_PUMPED_STORAGE\","]
#[doc = "    \"OTHER\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CtmDataNetworkGenItemPrimarySourceSubtype {
    #[serde(rename = "ANTRHC_BITMN_COAL")]
    AntrhcBitmnCoal,
    #[serde(rename = "WASTE_COAL")]
    WasteCoal,
    #[serde(rename = "DISTILLATE_FUEL_OIL")]
    DistillateFuelOil,
    #[serde(rename = "WASTE_OIL")]
    WasteOil,
    #[serde(rename = "PETROLEUM_COKE")]
    PetroleumCoke,
    #[serde(rename = "RESIDUAL_FUEL_OIL")]
    ResidualFuelOil,
    #[serde(rename = "NATURAL_GAS")]
    NaturalGas,
    #[serde(rename = "OTHER_GAS")]
    OtherGas,
    #[serde(rename = "NUCLEAR")]
    Nuclear,
    #[serde(rename = "AG_BIPRODUCT")]
    AgBiproduct,
    #[serde(rename = "MUNICIPAL_WASTE")]
    MunicipalWaste,
    #[serde(rename = "WOOD_WASTE")]
    WoodWaste,
    #[serde(rename = "GEOTHERMAL")]
    Geothermal,
    #[serde(rename = "SOLAR_PV")]
    SolarPv,
    #[serde(rename = "SOLAR_CSP")]
    SolarCsp,
    #[serde(rename = "WIND_ONSHORE")]
    WindOnshore,
    #[serde(rename = "WIND_OFFSHORE")]
    WindOffshore,
    #[serde(rename = "HYDRO_RUN_OF_THE_RIVER")]
    HydroRunOfTheRiver,
    #[serde(rename = "HYDRO_DAM")]
    HydroDam,
    #[serde(rename = "HYDRO_PUMPED_STORAGE")]
    HydroPumpedStorage,
    #[serde(rename = "OTHER")]
    Other,
}
impl ::std::convert::From<&Self> for CtmDataNetworkGenItemPrimarySourceSubtype {
    fn from(value: &CtmDataNetworkGenItemPrimarySourceSubtype) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CtmDataNetworkGenItemPrimarySourceSubtype {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AntrhcBitmnCoal => write!(f, "ANTRHC_BITMN_COAL"),
            Self::WasteCoal => write!(f, "WASTE_COAL"),
            Self::DistillateFuelOil => write!(f, "DISTILLATE_FUEL_OIL"),
            Self::WasteOil => write!(f, "WASTE_OIL"),
            Self::PetroleumCoke => write!(f, "PETROLEUM_COKE"),
            Self::ResidualFuelOil => write!(f, "RESIDUAL_FUEL_OIL"),
            Self::NaturalGas => write!(f, "NATURAL_GAS"),
            Self::OtherGas => write!(f, "OTHER_GAS"),
            Self::Nuclear => write!(f, "NUCLEAR"),
            Self::AgBiproduct => write!(f, "AG_BIPRODUCT"),
            Self::MunicipalWaste => write!(f, "MUNICIPAL_WASTE"),
            Self::WoodWaste => write!(f, "WOOD_WASTE"),
            Self::Geothermal => write!(f, "GEOTHERMAL"),
            Self::SolarPv => write!(f, "SOLAR_PV"),
            Self::SolarCsp => write!(f, "SOLAR_CSP"),
            Self::WindOnshore => write!(f, "WIND_ONSHORE"),
            Self::WindOffshore => write!(f, "WIND_OFFSHORE"),
            Self::HydroRunOfTheRiver => write!(f, "HYDRO_RUN_OF_THE_RIVER"),
            Self::HydroDam => write!(f, "HYDRO_DAM"),
            Self::HydroPumpedStorage => write!(f, "HYDRO_PUMPED_STORAGE"),
            Self::Other => write!(f, "OTHER"),
        }
    }
}
impl ::std::str::FromStr for CtmDataNetworkGenItemPrimarySourceSubtype {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ANTRHC_BITMN_COAL" => Ok(Self::AntrhcBitmnCoal),
            "WASTE_COAL" => Ok(Self::WasteCoal),
            "DISTILLATE_FUEL_OIL" => Ok(Self::DistillateFuelOil),
            "WASTE_OIL" => Ok(Self::WasteOil),
            "PETROLEUM_COKE" => Ok(Self::PetroleumCoke),
            "RESIDUAL_FUEL_OIL" => Ok(Self::ResidualFuelOil),
            "NATURAL_GAS" => Ok(Self::NaturalGas),
            "OTHER_GAS" => Ok(Self::OtherGas),
            "NUCLEAR" => Ok(Self::Nuclear),
            "AG_BIPRODUCT" => Ok(Self::AgBiproduct),
            "MUNICIPAL_WASTE" => Ok(Self::MunicipalWaste),
            "WOOD_WASTE" => Ok(Self::WoodWaste),
            "GEOTHERMAL" => Ok(Self::Geothermal),
            "SOLAR_PV" => Ok(Self::SolarPv),
            "SOLAR_CSP" => Ok(Self::SolarCsp),
            "WIND_ONSHORE" => Ok(Self::WindOnshore),
            "WIND_OFFSHORE" => Ok(Self::WindOffshore),
            "HYDRO_RUN_OF_THE_RIVER" => Ok(Self::HydroRunOfTheRiver),
            "HYDRO_DAM" => Ok(Self::HydroDam),
            "HYDRO_PUMPED_STORAGE" => Ok(Self::HydroPumpedStorage),
            "OTHER" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CtmDataNetworkGenItemPrimarySourceSubtype {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CtmDataNetworkGenItemPrimarySourceSubtype {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CtmDataNetworkGenItemPrimarySourceSubtype {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "[MVAr or pu] lower bound of reactive power injection (rectangular operating zone)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MVAr or pu] lower bound of reactive power injection (rectangular operating zone)\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkGenItemQgLb {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkGenItemQgLb {
    fn from(value: &CtmDataNetworkGenItemQgLb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkGenItemQgLb {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkGenItemQgLb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[MVAr or pu] upper bound of reactive power injection (rectangular operating zone)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MVAr or pu] upper bound of reactive power injection (rectangular operating zone)\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkGenItemQgUb {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkGenItemQgUb {
    fn from(value: &CtmDataNetworkGenItemQgUb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkGenItemQgUb {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkGenItemQgUb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "whether generator must be in service (e.g., nuclear power plant) or out of service (e.g., generator during maintenance or after an outage); 0 => no requirement, 1 => fixed in service, 2 => fixed out of service"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"whether generator must be in service (e.g., nuclear power plant) or out of service (e.g., generator during maintenance or after an outage); 0 => no requirement, 1 => fixed in service, 2 => fixed out of service\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 2.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkGenItemServiceRequired {
    Variant0(i64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkGenItemServiceRequired {
    fn from(value: &CtmDataNetworkGenItemServiceRequired) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for CtmDataNetworkGenItemServiceRequired {
    fn from(value: i64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkGenItemServiceRequired {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[$] cost of shutting down the unit"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[$] cost of shutting down the unit\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkGenItemShutdownCost {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkGenItemShutdownCost {
    fn from(value: &CtmDataNetworkGenItemShutdownCost) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkGenItemShutdownCost {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkGenItemShutdownCost {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[$] cost of starting the unit after being off > startup_time_warm hours"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[$] cost of starting the unit after being off > startup_time_warm hours\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkGenItemStartupCostCold {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkGenItemStartupCostCold {
    fn from(value: &CtmDataNetworkGenItemStartupCostCold) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkGenItemStartupCostCold {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkGenItemStartupCostCold {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[$] cost of starting the unit after being off <= startup_time_hot hours"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[$] cost of starting the unit after being off <= startup_time_hot hours\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkGenItemStartupCostHot {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkGenItemStartupCostHot {
    fn from(value: &CtmDataNetworkGenItemStartupCostHot) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkGenItemStartupCostHot {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkGenItemStartupCostHot {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[$] cost of starting the unit after being off > startup_time_hot hours, but <= startup_time_warm hours"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[$] cost of starting the unit after being off > startup_time_hot hours, but <= startup_time_warm hours\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkGenItemStartupCostWarm {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkGenItemStartupCostWarm {
    fn from(value: &CtmDataNetworkGenItemStartupCostWarm) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkGenItemStartupCostWarm {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkGenItemStartupCostWarm {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[kV or pu] target voltage magnitude of the bus that this generator connects to"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[kV or pu] target voltage magnitude of the bus that this generator connects to\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkGenItemVmSetpoint {
    PositiveNumber(PositiveNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkGenItemVmSetpoint {
    fn from(value: &CtmDataNetworkGenItemVmSetpoint) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<PositiveNumber> for CtmDataNetworkGenItemVmSetpoint {
    fn from(value: PositiveNumber) -> Self {
        Self::PositiveNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkGenItemVmSetpoint {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "structure to hold global settings for parameters in the network"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold global settings for parameters in the network\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"unit_convention\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"base_mva\": {"]
#[doc = "      \"description\": \"[MVA] system-wide apparent power base\","]
#[doc = "      \"default\": 100.0,"]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    \"bus_ref\": {"]
#[doc = "      \"description\": \"UID of reference bus of the electrical network\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"unit_convention\": {"]
#[doc = "      \"description\": \"units used for physical network parameters\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"NATURAL_UNITS\","]
#[doc = "        \"PER_UNIT_COMPONENT_BASE\","]
#[doc = "        \"PER_UNIT_SYSTEM_BASE\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataNetworkGlobalParams {
    #[doc = "[MVA] system-wide apparent power base"]
    #[serde(default = "defaults::ctm_data_network_global_params_base_mva")]
    pub base_mva: PositiveNumber,
    #[doc = "UID of reference bus of the electrical network"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bus_ref: ::std::option::Option<Uid>,
    #[doc = "units used for physical network parameters"]
    pub unit_convention: CtmDataNetworkGlobalParamsUnitConvention,
}
impl ::std::convert::From<&CtmDataNetworkGlobalParams> for CtmDataNetworkGlobalParams {
    fn from(value: &CtmDataNetworkGlobalParams) -> Self {
        value.clone()
    }
}
impl CtmDataNetworkGlobalParams {
    pub fn builder() -> builder::CtmDataNetworkGlobalParams {
        Default::default()
    }
}
#[doc = "units used for physical network parameters"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"units used for physical network parameters\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"NATURAL_UNITS\","]
#[doc = "    \"PER_UNIT_COMPONENT_BASE\","]
#[doc = "    \"PER_UNIT_SYSTEM_BASE\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CtmDataNetworkGlobalParamsUnitConvention {
    #[serde(rename = "NATURAL_UNITS")]
    NaturalUnits,
    #[serde(rename = "PER_UNIT_COMPONENT_BASE")]
    PerUnitComponentBase,
    #[serde(rename = "PER_UNIT_SYSTEM_BASE")]
    PerUnitSystemBase,
}
impl ::std::convert::From<&Self> for CtmDataNetworkGlobalParamsUnitConvention {
    fn from(value: &CtmDataNetworkGlobalParamsUnitConvention) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CtmDataNetworkGlobalParamsUnitConvention {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NaturalUnits => write!(f, "NATURAL_UNITS"),
            Self::PerUnitComponentBase => write!(f, "PER_UNIT_COMPONENT_BASE"),
            Self::PerUnitSystemBase => write!(f, "PER_UNIT_SYSTEM_BASE"),
        }
    }
}
impl ::std::str::FromStr for CtmDataNetworkGlobalParamsUnitConvention {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NATURAL_UNITS" => Ok(Self::NaturalUnits),
            "PER_UNIT_COMPONENT_BASE" => Ok(Self::PerUnitComponentBase),
            "PER_UNIT_SYSTEM_BASE" => Ok(Self::PerUnitSystemBase),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CtmDataNetworkGlobalParamsUnitConvention {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CtmDataNetworkGlobalParamsUnitConvention {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CtmDataNetworkGlobalParamsUnitConvention {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "structure to hold point-to-point hvdc line data"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold point-to-point hvdc line data\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bus_fr\","]
#[doc = "    \"bus_to\","]
#[doc = "    \"status\","]
#[doc = "    \"uid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"base_kv_dc\": {"]
#[doc = "      \"description\": \"[kV] base voltage at the dc side\","]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    \"bus_fr\": {"]
#[doc = "      \"description\": \"uid of bus at the from terminal of hvdc line\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"bus_to\": {"]
#[doc = "      \"description\": \"uid of bus at the to terminal of hvdc line\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"cm_ub_fr\": {"]
#[doc = "      \"description\": \"[kA or pu] ac persistent current rating, from terminal (if in pu, use from bus base_kv)\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cm_ub_to\": {"]
#[doc = "      \"description\": \"[kA or pu] ac persistent current rating, to terminal (if in pu, use to bus base_kv)\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional hvdc point-to-point parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"loss_a\": {"]
#[doc = "      \"description\": \"[MW or pu] standby loss\","]
#[doc = "      \"default\": 0,"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"loss_b\": {"]
#[doc = "      \"description\": \"[kV or pu] loss proportional to current magnitude (if in pu, base voltage corresponds to base_kv_dc)\","]
#[doc = "      \"default\": 0,"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"loss_c\": {"]
#[doc = "      \"description\": \"[Ohm or pu] loss proportional to current magnitude squared (if in pu, base voltage corresponds to base_kv_dc)\","]
#[doc = "      \"default\": 0,"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"HVDC line name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nominal_mva\": {"]
#[doc = "      \"description\": \"[MVA] nominal apparent power of hvdc line\","]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    \"p\": {"]
#[doc = "      \"description\": \"number of poles; 1 => monopole, 2 => bipole\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 2.0,"]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"pdc_fr_lb\": {"]
#[doc = "      \"description\": \"[MW or pu] minimum active power entering hvdc line at from bus\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pdc_fr_ub\": {"]
#[doc = "      \"description\": \"[MW or pu] maximum active power entering hvdc line at from bus\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pdc_to_lb\": {"]
#[doc = "      \"description\": \"[MW or pu] minimum active power entering hvdc line at to bus\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pdc_to_ub\": {"]
#[doc = "      \"description\": \"[MW or pu] maximum active power entering hvdc line at to bus\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"persistent_outage_duration\": {"]
#[doc = "      \"description\": \"[hours] expected duration of persistent outage (time between outage and crews re-energizing the branch)\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"persistent_outage_rate\": {"]
#[doc = "      \"description\": \"[events/year] number of expected persistent outages per year (outages not cleared by reconnectors)\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"phi_lb\": {"]
#[doc = "      \"description\": \"[deg] only meaningful if technology == LCC; firing angle minimum\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"phi_ub\": {"]
#[doc = "      \"description\": \"[deg] only meaningful if technology == LCC; firing angle maximum\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"qdc_fr_lb\": {"]
#[doc = "      \"description\": \"[MVAr or pu] minimum reactive power entering hvdc line at from bus\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"qdc_fr_ub\": {"]
#[doc = "      \"description\": \"[MVAr or pu] maximum reactive power entering hvdc line at from bus\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"qdc_to_lb\": {"]
#[doc = "      \"description\": \"[MVAr or pu] minimum reactive power entering hvdc line at to bus\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"qdc_to_ub\": {"]
#[doc = "      \"description\": \"[MW or pu] maximum active power entering hvdc line at to bus\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"r\": {"]
#[doc = "      \"description\": \"[Ohm or pu] dc line resistance (if in pu, base voltage corresponds to base_kv_dc)\","]
#[doc = "      \"default\": 0.0,"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"sm_ub\": {"]
#[doc = "      \"description\": \"[MVA or pu] ac persistent apparent power rating\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/$defs/status\""]
#[doc = "    },"]
#[doc = "    \"technology\": {"]
#[doc = "      \"description\": \"power conversion technology\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"LCC\","]
#[doc = "        \"VSC\","]
#[doc = "        \"MMC\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"transient_outage_rate\": {"]
#[doc = "      \"description\": \"[events/year] number of expected transient outages per year (outages cleared by reconnectors or other)\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"vm_dc_lb\": {"]
#[doc = "      \"description\": \"[kV or pu] minimum voltage at the dc side\","]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    \"vm_dc_ub\": {"]
#[doc = "      \"description\": \"[kV or pu] maximum voltage at the dc side\","]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataNetworkHvdcP2pItem {
    #[doc = "[kV] base voltage at the dc side"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub base_kv_dc: ::std::option::Option<PositiveNumber>,
    #[doc = "uid of bus at the from terminal of hvdc line"]
    pub bus_fr: Uid,
    #[doc = "uid of bus at the to terminal of hvdc line"]
    pub bus_to: Uid,
    #[doc = "[kA or pu] ac persistent current rating, from terminal (if in pu, use from bus base_kv)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cm_ub_fr: ::std::option::Option<CtmDataNetworkHvdcP2pItemCmUbFr>,
    #[doc = "[kA or pu] ac persistent current rating, to terminal (if in pu, use to bus base_kv)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cm_ub_to: ::std::option::Option<CtmDataNetworkHvdcP2pItemCmUbTo>,
    #[doc = "additional hvdc point-to-point parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    #[doc = "[MW or pu] standby loss"]
    #[serde(default = "defaults::ctm_data_network_hvdc_p2p_item_loss_a")]
    pub loss_a: NonnegativeNumber,
    #[doc = "[kV or pu] loss proportional to current magnitude (if in pu, base voltage corresponds to base_kv_dc)"]
    #[serde(default = "defaults::ctm_data_network_hvdc_p2p_item_loss_b")]
    pub loss_b: NonnegativeNumber,
    #[doc = "[Ohm or pu] loss proportional to current magnitude squared (if in pu, base voltage corresponds to base_kv_dc)"]
    #[serde(default = "defaults::ctm_data_network_hvdc_p2p_item_loss_c")]
    pub loss_c: NonnegativeNumber,
    #[doc = "HVDC line name"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "[MVA] nominal apparent power of hvdc line"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nominal_mva: ::std::option::Option<PositiveNumber>,
    #[doc = "number of poles; 1 => monopole, 2 => bipole"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub p: ::std::option::Option<i64>,
    #[doc = "[MW or pu] minimum active power entering hvdc line at from bus"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pdc_fr_lb: ::std::option::Option<CtmDataNetworkHvdcP2pItemPdcFrLb>,
    #[doc = "[MW or pu] maximum active power entering hvdc line at from bus"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pdc_fr_ub: ::std::option::Option<CtmDataNetworkHvdcP2pItemPdcFrUb>,
    #[doc = "[MW or pu] minimum active power entering hvdc line at to bus"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pdc_to_lb: ::std::option::Option<CtmDataNetworkHvdcP2pItemPdcToLb>,
    #[doc = "[MW or pu] maximum active power entering hvdc line at to bus"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pdc_to_ub: ::std::option::Option<CtmDataNetworkHvdcP2pItemPdcToUb>,
    #[doc = "[hours] expected duration of persistent outage (time between outage and crews re-energizing the branch)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub persistent_outage_duration: ::std::option::Option<NonnegativeNumber>,
    #[doc = "[events/year] number of expected persistent outages per year (outages not cleared by reconnectors)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub persistent_outage_rate: ::std::option::Option<NonnegativeNumber>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub phi_lb: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub phi_ub: ::std::option::Option<f64>,
    #[doc = "[MVAr or pu] minimum reactive power entering hvdc line at from bus"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub qdc_fr_lb: ::std::option::Option<CtmDataNetworkHvdcP2pItemQdcFrLb>,
    #[doc = "[MVAr or pu] maximum reactive power entering hvdc line at from bus"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub qdc_fr_ub: ::std::option::Option<CtmDataNetworkHvdcP2pItemQdcFrUb>,
    #[doc = "[MVAr or pu] minimum reactive power entering hvdc line at to bus"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub qdc_to_lb: ::std::option::Option<CtmDataNetworkHvdcP2pItemQdcToLb>,
    #[doc = "[MW or pu] maximum active power entering hvdc line at to bus"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub qdc_to_ub: ::std::option::Option<CtmDataNetworkHvdcP2pItemQdcToUb>,
    #[doc = "[Ohm or pu] dc line resistance (if in pu, base voltage corresponds to base_kv_dc)"]
    #[serde(default = "defaults::ctm_data_network_hvdc_p2p_item_r")]
    pub r: NonnegativeNumber,
    #[doc = "[MVA or pu] ac persistent apparent power rating"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sm_ub: ::std::option::Option<CtmDataNetworkHvdcP2pItemSmUb>,
    pub status: Status,
    #[doc = "power conversion technology"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub technology: ::std::option::Option<CtmDataNetworkHvdcP2pItemTechnology>,
    #[doc = "[events/year] number of expected transient outages per year (outages cleared by reconnectors or other)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub transient_outage_rate: ::std::option::Option<NonnegativeNumber>,
    pub uid: Uid,
    #[doc = "[kV or pu] minimum voltage at the dc side"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub vm_dc_lb: ::std::option::Option<PositiveNumber>,
    #[doc = "[kV or pu] maximum voltage at the dc side"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub vm_dc_ub: ::std::option::Option<PositiveNumber>,
}
impl ::std::convert::From<&CtmDataNetworkHvdcP2pItem> for CtmDataNetworkHvdcP2pItem {
    fn from(value: &CtmDataNetworkHvdcP2pItem) -> Self {
        value.clone()
    }
}
impl CtmDataNetworkHvdcP2pItem {
    pub fn builder() -> builder::CtmDataNetworkHvdcP2pItem {
        Default::default()
    }
}
#[doc = "[kA or pu] ac persistent current rating, from terminal (if in pu, use from bus base_kv)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[kA or pu] ac persistent current rating, from terminal (if in pu, use from bus base_kv)\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkHvdcP2pItemCmUbFr {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkHvdcP2pItemCmUbFr {
    fn from(value: &CtmDataNetworkHvdcP2pItemCmUbFr) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkHvdcP2pItemCmUbFr {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkHvdcP2pItemCmUbFr {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "[kA or pu] ac persistent current rating, to terminal (if in pu, use to bus base_kv)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[kA or pu] ac persistent current rating, to terminal (if in pu, use to bus base_kv)\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkHvdcP2pItemCmUbTo {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkHvdcP2pItemCmUbTo {
    fn from(value: &CtmDataNetworkHvdcP2pItemCmUbTo) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkHvdcP2pItemCmUbTo {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkHvdcP2pItemCmUbTo {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "[MW or pu] minimum active power entering hvdc line at from bus"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MW or pu] minimum active power entering hvdc line at from bus\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkHvdcP2pItemPdcFrLb {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkHvdcP2pItemPdcFrLb {
    fn from(value: &CtmDataNetworkHvdcP2pItemPdcFrLb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkHvdcP2pItemPdcFrLb {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkHvdcP2pItemPdcFrLb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[MW or pu] maximum active power entering hvdc line at from bus"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MW or pu] maximum active power entering hvdc line at from bus\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkHvdcP2pItemPdcFrUb {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkHvdcP2pItemPdcFrUb {
    fn from(value: &CtmDataNetworkHvdcP2pItemPdcFrUb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkHvdcP2pItemPdcFrUb {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkHvdcP2pItemPdcFrUb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[MW or pu] minimum active power entering hvdc line at to bus"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MW or pu] minimum active power entering hvdc line at to bus\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkHvdcP2pItemPdcToLb {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkHvdcP2pItemPdcToLb {
    fn from(value: &CtmDataNetworkHvdcP2pItemPdcToLb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkHvdcP2pItemPdcToLb {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkHvdcP2pItemPdcToLb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[MW or pu] maximum active power entering hvdc line at to bus"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MW or pu] maximum active power entering hvdc line at to bus\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkHvdcP2pItemPdcToUb {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkHvdcP2pItemPdcToUb {
    fn from(value: &CtmDataNetworkHvdcP2pItemPdcToUb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkHvdcP2pItemPdcToUb {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkHvdcP2pItemPdcToUb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[MVAr or pu] minimum reactive power entering hvdc line at from bus"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MVAr or pu] minimum reactive power entering hvdc line at from bus\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkHvdcP2pItemQdcFrLb {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkHvdcP2pItemQdcFrLb {
    fn from(value: &CtmDataNetworkHvdcP2pItemQdcFrLb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkHvdcP2pItemQdcFrLb {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkHvdcP2pItemQdcFrLb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[MVAr or pu] maximum reactive power entering hvdc line at from bus"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MVAr or pu] maximum reactive power entering hvdc line at from bus\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkHvdcP2pItemQdcFrUb {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkHvdcP2pItemQdcFrUb {
    fn from(value: &CtmDataNetworkHvdcP2pItemQdcFrUb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkHvdcP2pItemQdcFrUb {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkHvdcP2pItemQdcFrUb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[MVAr or pu] minimum reactive power entering hvdc line at to bus"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MVAr or pu] minimum reactive power entering hvdc line at to bus\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkHvdcP2pItemQdcToLb {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkHvdcP2pItemQdcToLb {
    fn from(value: &CtmDataNetworkHvdcP2pItemQdcToLb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkHvdcP2pItemQdcToLb {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkHvdcP2pItemQdcToLb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[MW or pu] maximum active power entering hvdc line at to bus"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MW or pu] maximum active power entering hvdc line at to bus\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkHvdcP2pItemQdcToUb {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkHvdcP2pItemQdcToUb {
    fn from(value: &CtmDataNetworkHvdcP2pItemQdcToUb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkHvdcP2pItemQdcToUb {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkHvdcP2pItemQdcToUb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[MVA or pu] ac persistent apparent power rating"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MVA or pu] ac persistent apparent power rating\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkHvdcP2pItemSmUb {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkHvdcP2pItemSmUb {
    fn from(value: &CtmDataNetworkHvdcP2pItemSmUb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkHvdcP2pItemSmUb {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkHvdcP2pItemSmUb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "power conversion technology"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"power conversion technology\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"LCC\","]
#[doc = "    \"VSC\","]
#[doc = "    \"MMC\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CtmDataNetworkHvdcP2pItemTechnology {
    #[serde(rename = "LCC")]
    Lcc,
    #[serde(rename = "VSC")]
    Vsc,
    #[serde(rename = "MMC")]
    Mmc,
}
impl ::std::convert::From<&Self> for CtmDataNetworkHvdcP2pItemTechnology {
    fn from(value: &CtmDataNetworkHvdcP2pItemTechnology) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CtmDataNetworkHvdcP2pItemTechnology {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Lcc => write!(f, "LCC"),
            Self::Vsc => write!(f, "VSC"),
            Self::Mmc => write!(f, "MMC"),
        }
    }
}
impl ::std::str::FromStr for CtmDataNetworkHvdcP2pItemTechnology {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "LCC" => Ok(Self::Lcc),
            "VSC" => Ok(Self::Vsc),
            "MMC" => Ok(Self::Mmc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CtmDataNetworkHvdcP2pItemTechnology {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CtmDataNetworkHvdcP2pItemTechnology {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CtmDataNetworkHvdcP2pItemTechnology {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "structure to hold load (consumer) data using ZIP model"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold load (consumer) data using ZIP model\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bus\","]
#[doc = "    \"pd\","]
#[doc = "    \"qd\","]
#[doc = "    \"status\","]
#[doc = "    \"uid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bus\": {"]
#[doc = "      \"description\": \"uid of bus to which load is connected to\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional bus parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"load name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nominal_mva\": {"]
#[doc = "      \"description\": \"[MVA] nominal power of load\","]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    \"pd\": {"]
#[doc = "      \"description\": \"active power demand\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pd_i\": {"]
#[doc = "      \"description\": \"constant current active power demand at v_bus = 1.0 pu\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pd_y\": {"]
#[doc = "      \"description\": \"constant impedance active power demand at v_bus = 1.0 pu\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"qd\": {"]
#[doc = "      \"description\": \"reactive power demand\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"qd_i\": {"]
#[doc = "      \"description\": \"constant current reactive power demand at v_bus = 1.0 pu\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"qd_y\": {"]
#[doc = "      \"description\": \"constant impedance reactive power demand at v_bus = 1.0 pu\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/$defs/status\""]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataNetworkLoadItem {
    #[doc = "uid of bus to which load is connected to"]
    pub bus: Uid,
    #[doc = "additional bus parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    #[doc = "load name"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "[MVA] nominal power of load"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nominal_mva: ::std::option::Option<PositiveNumber>,
    #[doc = "active power demand"]
    pub pd: CtmDataNetworkLoadItemPd,
    #[doc = "constant current active power demand at v_bus = 1.0 pu"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pd_i: ::std::option::Option<CtmDataNetworkLoadItemPdI>,
    #[doc = "constant impedance active power demand at v_bus = 1.0 pu"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pd_y: ::std::option::Option<CtmDataNetworkLoadItemPdY>,
    #[doc = "reactive power demand"]
    pub qd: CtmDataNetworkLoadItemQd,
    #[doc = "constant current reactive power demand at v_bus = 1.0 pu"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub qd_i: ::std::option::Option<CtmDataNetworkLoadItemQdI>,
    #[doc = "constant impedance reactive power demand at v_bus = 1.0 pu"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub qd_y: ::std::option::Option<CtmDataNetworkLoadItemQdY>,
    pub status: Status,
    pub uid: Uid,
}
impl ::std::convert::From<&CtmDataNetworkLoadItem> for CtmDataNetworkLoadItem {
    fn from(value: &CtmDataNetworkLoadItem) -> Self {
        value.clone()
    }
}
impl CtmDataNetworkLoadItem {
    pub fn builder() -> builder::CtmDataNetworkLoadItem {
        Default::default()
    }
}
#[doc = "active power demand"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"active power demand\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkLoadItemPd {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkLoadItemPd {
    fn from(value: &CtmDataNetworkLoadItemPd) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkLoadItemPd {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkLoadItemPd {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "constant current active power demand at v_bus = 1.0 pu"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"constant current active power demand at v_bus = 1.0 pu\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkLoadItemPdI {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkLoadItemPdI {
    fn from(value: &CtmDataNetworkLoadItemPdI) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkLoadItemPdI {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkLoadItemPdI {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "constant impedance active power demand at v_bus = 1.0 pu"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"constant impedance active power demand at v_bus = 1.0 pu\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkLoadItemPdY {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkLoadItemPdY {
    fn from(value: &CtmDataNetworkLoadItemPdY) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkLoadItemPdY {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkLoadItemPdY {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "reactive power demand"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"reactive power demand\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkLoadItemQd {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkLoadItemQd {
    fn from(value: &CtmDataNetworkLoadItemQd) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkLoadItemQd {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkLoadItemQd {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "constant current reactive power demand at v_bus = 1.0 pu"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"constant current reactive power demand at v_bus = 1.0 pu\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkLoadItemQdI {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkLoadItemQdI {
    fn from(value: &CtmDataNetworkLoadItemQdI) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkLoadItemQdI {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkLoadItemQdI {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "constant impedance reactive power demand at v_bus = 1.0 pu"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"constant impedance reactive power demand at v_bus = 1.0 pu\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkLoadItemQdY {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkLoadItemQdY {
    fn from(value: &CtmDataNetworkLoadItemQdY) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkLoadItemQdY {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkLoadItemQdY {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "structure to hold reserve product and requirement data"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold reserve product and requirement data\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"reserve_type\","]
#[doc = "    \"status\","]
#[doc = "    \"uid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional reserve parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"name of reserve product\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"participants\": {"]
#[doc = "      \"description\": \"uid of generators contributing to this reserve\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/uid\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"pg_down\": {"]
#[doc = "      \"description\": \"[MW or pu] downward active power required by this reserve\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pg_up\": {"]
#[doc = "      \"description\": \"[MW or pu] upward active power required by this reserve\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"reserve_type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"PRIMARY\","]
#[doc = "        \"SECONDARY\","]
#[doc = "        \"TERTIARY\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/$defs/status\""]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataNetworkReserveItem {
    #[doc = "additional reserve parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    #[doc = "name of reserve product"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "uid of generators contributing to this reserve"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub participants: ::std::vec::Vec<Uid>,
    #[doc = "[MW or pu] downward active power required by this reserve"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pg_down: ::std::option::Option<CtmDataNetworkReserveItemPgDown>,
    #[doc = "[MW or pu] upward active power required by this reserve"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pg_up: ::std::option::Option<CtmDataNetworkReserveItemPgUp>,
    pub reserve_type: CtmDataNetworkReserveItemReserveType,
    pub status: Status,
    pub uid: Uid,
}
impl ::std::convert::From<&CtmDataNetworkReserveItem> for CtmDataNetworkReserveItem {
    fn from(value: &CtmDataNetworkReserveItem) -> Self {
        value.clone()
    }
}
impl CtmDataNetworkReserveItem {
    pub fn builder() -> builder::CtmDataNetworkReserveItem {
        Default::default()
    }
}
#[doc = "[MW or pu] downward active power required by this reserve"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MW or pu] downward active power required by this reserve\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkReserveItemPgDown {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkReserveItemPgDown {
    fn from(value: &CtmDataNetworkReserveItemPgDown) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkReserveItemPgDown {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkReserveItemPgDown {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "[MW or pu] upward active power required by this reserve"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MW or pu] upward active power required by this reserve\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkReserveItemPgUp {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkReserveItemPgUp {
    fn from(value: &CtmDataNetworkReserveItemPgUp) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkReserveItemPgUp {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkReserveItemPgUp {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "`CtmDataNetworkReserveItemReserveType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"PRIMARY\","]
#[doc = "    \"SECONDARY\","]
#[doc = "    \"TERTIARY\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CtmDataNetworkReserveItemReserveType {
    #[serde(rename = "PRIMARY")]
    Primary,
    #[serde(rename = "SECONDARY")]
    Secondary,
    #[serde(rename = "TERTIARY")]
    Tertiary,
}
impl ::std::convert::From<&Self> for CtmDataNetworkReserveItemReserveType {
    fn from(value: &CtmDataNetworkReserveItemReserveType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CtmDataNetworkReserveItemReserveType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Primary => write!(f, "PRIMARY"),
            Self::Secondary => write!(f, "SECONDARY"),
            Self::Tertiary => write!(f, "TERTIARY"),
        }
    }
}
impl ::std::str::FromStr for CtmDataNetworkReserveItemReserveType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "PRIMARY" => Ok(Self::Primary),
            "SECONDARY" => Ok(Self::Secondary),
            "TERTIARY" => Ok(Self::Tertiary),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CtmDataNetworkReserveItemReserveType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CtmDataNetworkReserveItemReserveType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CtmDataNetworkReserveItemReserveType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "structure to hold shunt data"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold shunt data\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bs\","]
#[doc = "    \"bus\","]
#[doc = "    \"gs\","]
#[doc = "    \"num_steps_ub\","]
#[doc = "    \"status\","]
#[doc = "    \"uid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bs\": {"]
#[doc = "      \"description\": \"[MVAr or pu] reactive power demand at v_bus = 1.0 pu, per step of each shunt section\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"number\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"bus\": {"]
#[doc = "      \"description\": \"uid of bus to which shunt is connected to\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional shunt parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"gs\": {"]
#[doc = "      \"description\": \"[MW or pu] active power demand at v_bus = 1.0 pu, per step of each shunt section\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"shunt name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nominal_mva\": {"]
#[doc = "      \"description\": \"[MVA] nominal apparent power of shunt (nameplate capacity)\","]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    \"num_steps_ub\": {"]
#[doc = "      \"description\": \"upper bound for number of energized steps of shunt section (lower bound is always 0)\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_integer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_integer\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/$defs/status\""]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataNetworkShuntItem {
    #[doc = "[MVAr or pu] reactive power demand at v_bus = 1.0 pu, per step of each shunt section"]
    pub bs: CtmDataNetworkShuntItemBs,
    #[doc = "uid of bus to which shunt is connected to"]
    pub bus: Uid,
    #[doc = "additional shunt parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    #[doc = "[MW or pu] active power demand at v_bus = 1.0 pu, per step of each shunt section"]
    pub gs: CtmDataNetworkShuntItemGs,
    #[doc = "shunt name"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "[MVA] nominal apparent power of shunt (nameplate capacity)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nominal_mva: ::std::option::Option<PositiveNumber>,
    #[doc = "upper bound for number of energized steps of shunt section (lower bound is always 0)"]
    pub num_steps_ub: CtmDataNetworkShuntItemNumStepsUb,
    pub status: Status,
    pub uid: Uid,
}
impl ::std::convert::From<&CtmDataNetworkShuntItem> for CtmDataNetworkShuntItem {
    fn from(value: &CtmDataNetworkShuntItem) -> Self {
        value.clone()
    }
}
impl CtmDataNetworkShuntItem {
    pub fn builder() -> builder::CtmDataNetworkShuntItem {
        Default::default()
    }
}
#[doc = "[MVAr or pu] reactive power demand at v_bus = 1.0 pu, per step of each shunt section"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MVAr or pu] reactive power demand at v_bus = 1.0 pu, per step of each shunt section\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkShuntItemBs {
    Variant0(f64),
    Variant1(::std::vec::Vec<f64>),
}
impl ::std::convert::From<&Self> for CtmDataNetworkShuntItemBs {
    fn from(value: &CtmDataNetworkShuntItemBs) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkShuntItemBs {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<f64>> for CtmDataNetworkShuntItemBs {
    fn from(value: ::std::vec::Vec<f64>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[MW or pu] active power demand at v_bus = 1.0 pu, per step of each shunt section"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MW or pu] active power demand at v_bus = 1.0 pu, per step of each shunt section\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkShuntItemGs {
    Variant0(NonnegativeNumber),
    Variant1(::std::vec::Vec<NonnegativeNumber>),
}
impl ::std::convert::From<&Self> for CtmDataNetworkShuntItemGs {
    fn from(value: &CtmDataNetworkShuntItemGs) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkShuntItemGs {
    fn from(value: NonnegativeNumber) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<NonnegativeNumber>> for CtmDataNetworkShuntItemGs {
    fn from(value: ::std::vec::Vec<NonnegativeNumber>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "upper bound for number of energized steps of shunt section (lower bound is always 0)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"upper bound for number of energized steps of shunt section (lower bound is always 0)\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_integer\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/nonnegative_integer\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkShuntItemNumStepsUb {
    Variant0(NonnegativeInteger),
    Variant1(::std::vec::Vec<NonnegativeInteger>),
}
impl ::std::convert::From<&Self> for CtmDataNetworkShuntItemNumStepsUb {
    fn from(value: &CtmDataNetworkShuntItemNumStepsUb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeInteger> for CtmDataNetworkShuntItemNumStepsUb {
    fn from(value: NonnegativeInteger) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<NonnegativeInteger>>
    for CtmDataNetworkShuntItemNumStepsUb
{
    fn from(value: ::std::vec::Vec<NonnegativeInteger>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "structure to hold storage (battery) data"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold storage (battery) data\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bus\","]
#[doc = "    \"charge_efficiency\","]
#[doc = "    \"discharge_efficiency\","]
#[doc = "    \"ps_ex\","]
#[doc = "    \"qs_ex\","]
#[doc = "    \"status\","]
#[doc = "    \"uid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bus\": {"]
#[doc = "      \"description\": \"uid of bus to which generator is connected to\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"charge_efficiency\": {"]
#[doc = "      \"description\": \"[-] charge efficiency, in (0, 1]\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"maximum\": 1.0,"]
#[doc = "          \"exclusiveMinimum\": 0.0"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"charge_ub\": {"]
#[doc = "      \"description\": \"[MW or pu] maximum rate of charge\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cm_ub\": {"]
#[doc = "      \"description\": \"[kA or pu] converter current output rating\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"discharge_efficiency\": {"]
#[doc = "      \"description\": \"[-] discharge efficiency, in (0, 1]\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"maximum\": 1.0,"]
#[doc = "          \"exclusiveMinimum\": 0.0"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"discharge_ub\": {"]
#[doc = "      \"description\": \"[MW or pu] maximum rate of discharge\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"energy_ub\": {"]
#[doc = "      \"description\": \"[MWh or pu*h] maximum state of charge\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional storage parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"storage name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nominal_mva\": {"]
#[doc = "      \"description\": \"[MVA] nominal apparent power of storage (nameplate capacity)\","]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    \"ps_delta_lb\": {"]
#[doc = "      \"description\": \"[MW/h or pu/h] maximum active power decrease per hour\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"ps_delta_ub\": {"]
#[doc = "      \"description\": \"[MW/h or pu/h] maximum active power increase per hour\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"ps_ex\": {"]
#[doc = "      \"description\": \"converter standby active power exogenous draw\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"qs_ex\": {"]
#[doc = "      \"description\": \"converter standby reactive power exogenous draw\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"qs_lb\": {"]
#[doc = "      \"description\": \"[MVAr or pu] minumum reactive power injection\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"qs_ub\": {"]
#[doc = "      \"description\": \"[MVAr or pu] maximum reactive power injection\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sm_ub\": {"]
#[doc = "      \"description\": \"[MVA or pu] converter apparent power rating\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/$defs/status\""]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataNetworkStorageItem {
    #[doc = "uid of bus to which generator is connected to"]
    pub bus: Uid,
    #[doc = "[-] charge efficiency, in (0, 1]"]
    pub charge_efficiency: CtmDataNetworkStorageItemChargeEfficiency,
    #[doc = "[MW or pu] maximum rate of charge"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub charge_ub: ::std::option::Option<CtmDataNetworkStorageItemChargeUb>,
    #[doc = "[kA or pu] converter current output rating"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cm_ub: ::std::option::Option<NonnegativeNumber>,
    #[doc = "[-] discharge efficiency, in (0, 1]"]
    pub discharge_efficiency: CtmDataNetworkStorageItemDischargeEfficiency,
    #[doc = "[MW or pu] maximum rate of discharge"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub discharge_ub: ::std::option::Option<CtmDataNetworkStorageItemDischargeUb>,
    #[doc = "[MWh or pu*h] maximum state of charge"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub energy_ub: ::std::option::Option<NonnegativeNumber>,
    #[doc = "additional storage parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    #[doc = "storage name"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "[MVA] nominal apparent power of storage (nameplate capacity)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nominal_mva: ::std::option::Option<PositiveNumber>,
    #[doc = "[MW/h or pu/h] maximum active power decrease per hour"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ps_delta_lb: ::std::option::Option<NonnegativeNumber>,
    #[doc = "[MW/h or pu/h] maximum active power increase per hour"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ps_delta_ub: ::std::option::Option<NonnegativeNumber>,
    pub ps_ex: f64,
    pub qs_ex: f64,
    #[doc = "[MVAr or pu] minumum reactive power injection"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub qs_lb: ::std::option::Option<CtmDataNetworkStorageItemQsLb>,
    #[doc = "[MVAr or pu] maximum reactive power injection"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub qs_ub: ::std::option::Option<CtmDataNetworkStorageItemQsUb>,
    #[doc = "[MVA or pu] converter apparent power rating"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sm_ub: ::std::option::Option<NonnegativeNumber>,
    pub status: Status,
    pub uid: Uid,
}
impl ::std::convert::From<&CtmDataNetworkStorageItem> for CtmDataNetworkStorageItem {
    fn from(value: &CtmDataNetworkStorageItem) -> Self {
        value.clone()
    }
}
impl CtmDataNetworkStorageItem {
    pub fn builder() -> builder::CtmDataNetworkStorageItem {
        Default::default()
    }
}
#[doc = "[-] charge efficiency, in (0, 1]"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[-] charge efficiency, in (0, 1]\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"exclusiveMinimum\": 0.0"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkStorageItemChargeEfficiency {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkStorageItemChargeEfficiency {
    fn from(value: &CtmDataNetworkStorageItemChargeEfficiency) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkStorageItemChargeEfficiency {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkStorageItemChargeEfficiency {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[MW or pu] maximum rate of charge"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MW or pu] maximum rate of charge\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkStorageItemChargeUb {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkStorageItemChargeUb {
    fn from(value: &CtmDataNetworkStorageItemChargeUb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkStorageItemChargeUb {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkStorageItemChargeUb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "[-] discharge efficiency, in (0, 1]"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[-] discharge efficiency, in (0, 1]\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"exclusiveMinimum\": 0.0"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkStorageItemDischargeEfficiency {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkStorageItemDischargeEfficiency {
    fn from(value: &CtmDataNetworkStorageItemDischargeEfficiency) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkStorageItemDischargeEfficiency {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkStorageItemDischargeEfficiency {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[MW or pu] maximum rate of discharge"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MW or pu] maximum rate of discharge\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkStorageItemDischargeUb {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkStorageItemDischargeUb {
    fn from(value: &CtmDataNetworkStorageItemDischargeUb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkStorageItemDischargeUb {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkStorageItemDischargeUb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "[MVAr or pu] minumum reactive power injection"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MVAr or pu] minumum reactive power injection\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkStorageItemQsLb {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkStorageItemQsLb {
    fn from(value: &CtmDataNetworkStorageItemQsLb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkStorageItemQsLb {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkStorageItemQsLb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "[MVAr or pu] maximum reactive power injection"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MVAr or pu] maximum reactive power injection\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkStorageItemQsUb {
    Variant0(f64),
    Variant1(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkStorageItemQsUb {
    fn from(value: &CtmDataNetworkStorageItemQsUb) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for CtmDataNetworkStorageItemQsUb {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkStorageItemQsUb {
    fn from(value: TimeSeriesReference) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "`CtmDataNetworkSwitchItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bus_fr\","]
#[doc = "    \"bus_to\","]
#[doc = "    \"status\","]
#[doc = "    \"uid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bus_fr\": {"]
#[doc = "      \"description\": \"uid of bus at the from terminal of switch\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"bus_to\": {"]
#[doc = "      \"description\": \"uid of bus at the to terminal of switch\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"cm_ub\": {"]
#[doc = "      \"description\": \"[kA or pu] current limit\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional switch parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"name of switch\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nominal_mva\": {"]
#[doc = "      \"description\": \"[MVA] nominal apparent power of switch (nameplate capacity)\","]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    \"sm_ub\": {"]
#[doc = "      \"description\": \"[MVA or pu] apparent power flow limit\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/$defs/status\""]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataNetworkSwitchItem {
    #[doc = "uid of bus at the from terminal of switch"]
    pub bus_fr: Uid,
    #[doc = "uid of bus at the to terminal of switch"]
    pub bus_to: Uid,
    #[doc = "[kA or pu] current limit"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cm_ub: ::std::option::Option<NonnegativeNumber>,
    #[doc = "additional switch parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    #[doc = "name of switch"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "[MVA] nominal apparent power of switch (nameplate capacity)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nominal_mva: ::std::option::Option<PositiveNumber>,
    #[doc = "[MVA or pu] apparent power flow limit"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sm_ub: ::std::option::Option<NonnegativeNumber>,
    pub status: Status,
    pub uid: Uid,
}
impl ::std::convert::From<&CtmDataNetworkSwitchItem> for CtmDataNetworkSwitchItem {
    fn from(value: &CtmDataNetworkSwitchItem) -> Self {
        value.clone()
    }
}
impl CtmDataNetworkSwitchItem {
    pub fn builder() -> builder::CtmDataNetworkSwitchItem {
        Default::default()
    }
}
#[doc = "structure to hold 2-winding transformer and phase shifter data using simplified (4-parameter circuit) model"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold 2-winding transformer and phase shifter data using simplified (4-parameter circuit) model\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"b\","]
#[doc = "    \"bus_fr\","]
#[doc = "    \"bus_to\","]
#[doc = "    \"g\","]
#[doc = "    \"r\","]
#[doc = "    \"status\","]
#[doc = "    \"uid\","]
#[doc = "    \"x\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"b\": {"]
#[doc = "      \"description\": \"[S or pu] shunt susceptance of transformer at from terminal (magnetizing branch)\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"bus_fr\": {"]
#[doc = "      \"description\": \"uid of bus at the from terminal of transformer\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"bus_to\": {"]
#[doc = "      \"description\": \"uid of bus at the to terminal of transformer\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"cm_ub_a\": {"]
#[doc = "      \"description\": \"[kA or pu] persistent current rating, referred to from side\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cm_ub_b\": {"]
#[doc = "      \"description\": \"[kA or pu] 4-hour current rating, referred to from side\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cm_ub_c\": {"]
#[doc = "      \"description\": \"[kA or pu] 15-minute current rating, referred to from side\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional transformer parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"g\": {"]
#[doc = "      \"description\": \"[S or pu] shunt conductance of transformer at from terminal (magnetizing branch)\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"transformer name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nominal_mva\": {"]
#[doc = "      \"description\": \"[MVA] nominal apparent power of transformer\","]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    \"persistent_outage_duration\": {"]
#[doc = "      \"description\": \"[hours] expected duration of persistent outage (time between outage and crews re-energizing the branch)\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"persistent_outage_rate\": {"]
#[doc = "      \"description\": \"[events/year] number of expected persistent outages per year (outages not cleared by reconnectors)\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"r\": {"]
#[doc = "      \"description\": \"[Ohm or pu] series resistance of line\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"sm_ub_a\": {"]
#[doc = "      \"description\": \"[MVA or pu] persistent apparent power rating, referred to from side\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sm_ub_b\": {"]
#[doc = "      \"description\": \"[MVA or pu] 4-hour apparent power rating, referred to from side\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sm_ub_c\": {"]
#[doc = "      \"description\": \"[MVA or pu] 15-minute apparent power rating, referred to from side\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/$defs/status\""]
#[doc = "    },"]
#[doc = "    \"ta_lb\": {"]
#[doc = "      \"description\": \"[deg] minimum angle phase shift (angle difference = va_from - va_to - angle_shift)\","]
#[doc = "      \"default\": 0,"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"ta_steps\": {"]
#[doc = "      \"description\": \"number of discrete steps between ta_lb and ta_ub (including limit values)\","]
#[doc = "      \"default\": 1,"]
#[doc = "      \"$ref\": \"#/$defs/positive_integer\""]
#[doc = "    },"]
#[doc = "    \"ta_ub\": {"]
#[doc = "      \"description\": \"[deg] maximum angle phase shift (angle difference = va_from - va_to - angle_shift)\","]
#[doc = "      \"default\": 0,"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"tm_lb\": {"]
#[doc = "      \"description\": \"[-] minimum tap ratio (1.0 correspond to nominal ratio, inner_vm_from = vm_from * tap_value)\","]
#[doc = "      \"default\": 1.0,"]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    \"tm_steps\": {"]
#[doc = "      \"description\": \"number of discrete steps between tm_lb and tm_ub (including limit values)\","]
#[doc = "      \"default\": 1,"]
#[doc = "      \"$ref\": \"#/$defs/positive_integer\""]
#[doc = "    },"]
#[doc = "    \"tm_ub\": {"]
#[doc = "      \"description\": \"[-] maximum tap ratio (1.0 correspond to nominal ratio, inner_vm_from = vm_from * tap_value)\","]
#[doc = "      \"default\": 1.0,"]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"x\": {"]
#[doc = "      \"description\": \"[Ohm or pu] series impedance of line\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataNetworkTransformerItem {
    pub b: f64,
    #[doc = "uid of bus at the from terminal of transformer"]
    pub bus_fr: Uid,
    #[doc = "uid of bus at the to terminal of transformer"]
    pub bus_to: Uid,
    #[doc = "[kA or pu] persistent current rating, referred to from side"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cm_ub_a: ::std::option::Option<CtmDataNetworkTransformerItemCmUbA>,
    #[doc = "[kA or pu] 4-hour current rating, referred to from side"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cm_ub_b: ::std::option::Option<CtmDataNetworkTransformerItemCmUbB>,
    #[doc = "[kA or pu] 15-minute current rating, referred to from side"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cm_ub_c: ::std::option::Option<CtmDataNetworkTransformerItemCmUbC>,
    #[doc = "additional transformer parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    pub g: f64,
    #[doc = "transformer name"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "[MVA] nominal apparent power of transformer"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nominal_mva: ::std::option::Option<PositiveNumber>,
    #[doc = "[hours] expected duration of persistent outage (time between outage and crews re-energizing the branch)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub persistent_outage_duration: ::std::option::Option<NonnegativeNumber>,
    #[doc = "[events/year] number of expected persistent outages per year (outages not cleared by reconnectors)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub persistent_outage_rate: ::std::option::Option<NonnegativeNumber>,
    pub r: f64,
    #[doc = "[MVA or pu] persistent apparent power rating, referred to from side"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sm_ub_a: ::std::option::Option<CtmDataNetworkTransformerItemSmUbA>,
    #[doc = "[MVA or pu] 4-hour apparent power rating, referred to from side"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sm_ub_b: ::std::option::Option<CtmDataNetworkTransformerItemSmUbB>,
    #[doc = "[MVA or pu] 15-minute apparent power rating, referred to from side"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sm_ub_c: ::std::option::Option<CtmDataNetworkTransformerItemSmUbC>,
    pub status: Status,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ta_lb: ::std::option::Option<f64>,
    #[doc = "number of discrete steps between ta_lb and ta_ub (including limit values)"]
    #[serde(default = "defaults::ctm_data_network_transformer_item_ta_steps")]
    pub ta_steps: PositiveInteger,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ta_ub: ::std::option::Option<f64>,
    #[doc = "[-] minimum tap ratio (1.0 correspond to nominal ratio, inner_vm_from = vm_from * tap_value)"]
    #[serde(default = "defaults::ctm_data_network_transformer_item_tm_lb")]
    pub tm_lb: PositiveNumber,
    #[doc = "number of discrete steps between tm_lb and tm_ub (including limit values)"]
    #[serde(default = "defaults::ctm_data_network_transformer_item_tm_steps")]
    pub tm_steps: PositiveInteger,
    #[doc = "[-] maximum tap ratio (1.0 correspond to nominal ratio, inner_vm_from = vm_from * tap_value)"]
    #[serde(default = "defaults::ctm_data_network_transformer_item_tm_ub")]
    pub tm_ub: PositiveNumber,
    pub uid: Uid,
    pub x: f64,
}
impl ::std::convert::From<&CtmDataNetworkTransformerItem> for CtmDataNetworkTransformerItem {
    fn from(value: &CtmDataNetworkTransformerItem) -> Self {
        value.clone()
    }
}
impl CtmDataNetworkTransformerItem {
    pub fn builder() -> builder::CtmDataNetworkTransformerItem {
        Default::default()
    }
}
#[doc = "[kA or pu] persistent current rating, referred to from side"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[kA or pu] persistent current rating, referred to from side\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkTransformerItemCmUbA {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkTransformerItemCmUbA {
    fn from(value: &CtmDataNetworkTransformerItemCmUbA) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkTransformerItemCmUbA {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkTransformerItemCmUbA {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "[kA or pu] 4-hour current rating, referred to from side"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[kA or pu] 4-hour current rating, referred to from side\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkTransformerItemCmUbB {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkTransformerItemCmUbB {
    fn from(value: &CtmDataNetworkTransformerItemCmUbB) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkTransformerItemCmUbB {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkTransformerItemCmUbB {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "[kA or pu] 15-minute current rating, referred to from side"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[kA or pu] 15-minute current rating, referred to from side\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkTransformerItemCmUbC {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkTransformerItemCmUbC {
    fn from(value: &CtmDataNetworkTransformerItemCmUbC) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkTransformerItemCmUbC {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkTransformerItemCmUbC {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "[MVA or pu] persistent apparent power rating, referred to from side"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MVA or pu] persistent apparent power rating, referred to from side\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkTransformerItemSmUbA {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkTransformerItemSmUbA {
    fn from(value: &CtmDataNetworkTransformerItemSmUbA) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkTransformerItemSmUbA {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkTransformerItemSmUbA {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "[MVA or pu] 4-hour apparent power rating, referred to from side"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MVA or pu] 4-hour apparent power rating, referred to from side\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkTransformerItemSmUbB {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkTransformerItemSmUbB {
    fn from(value: &CtmDataNetworkTransformerItemSmUbB) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkTransformerItemSmUbB {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkTransformerItemSmUbB {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "[MVA or pu] 15-minute apparent power rating, referred to from side"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[MVA or pu] 15-minute apparent power rating, referred to from side\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/time_series_reference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataNetworkTransformerItemSmUbC {
    NonnegativeNumber(NonnegativeNumber),
    TimeSeriesReference(TimeSeriesReference),
}
impl ::std::convert::From<&Self> for CtmDataNetworkTransformerItemSmUbC {
    fn from(value: &CtmDataNetworkTransformerItemSmUbC) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeNumber> for CtmDataNetworkTransformerItemSmUbC {
    fn from(value: NonnegativeNumber) -> Self {
        Self::NonnegativeNumber(value)
    }
}
impl ::std::convert::From<TimeSeriesReference> for CtmDataNetworkTransformerItemSmUbC {
    fn from(value: TimeSeriesReference) -> Self {
        Self::TimeSeriesReference(value)
    }
}
#[doc = "geographical subset of the electrical network commonly associated with market purposes (e.g., define sub-markets within a large interconnected system, defining different areas for reserve products, etc.)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"geographical subset of the electrical network commonly associated with market purposes (e.g., define sub-markets within a large interconnected system, defining different areas for reserve products, etc.)\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"status\","]
#[doc = "    \"uid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional zone parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"zone name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"binary indicator of whether zone should be included or omitted (if omitted all elements within zone should be omitted); 1=>included, 0=>omitted\","]
#[doc = "      \"$ref\": \"#/$defs/status\""]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataNetworkZoneItem {
    #[doc = "additional zone parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    #[doc = "zone name"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "binary indicator of whether zone should be included or omitted (if omitted all elements within zone should be omitted); 1=>included, 0=>omitted"]
    pub status: Status,
    pub uid: Uid,
}
impl ::std::convert::From<&CtmDataNetworkZoneItem> for CtmDataNetworkZoneItem {
    fn from(value: &CtmDataNetworkZoneItem) -> Self {
        value.clone()
    }
}
impl CtmDataNetworkZoneItem {
    pub fn builder() -> builder::CtmDataNetworkZoneItem {
        Default::default()
    }
}
#[doc = "structure to hold data on initial conditions of power system (state prior to start of time series data)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold data on initial conditions of power system (state prior to start of time series data)\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"global_params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bus\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"structure to hold initial state of bus variables\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"uid\","]
#[doc = "          \"va\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional bus initial condition parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"description\": \"uid of bus this record refers to\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"va\": {"]
#[doc = "            \"description\": \"[deg] initial voltage angle\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"vm\": {"]
#[doc = "            \"description\": \"[kV or pu] initial voltage magnitude\","]
#[doc = "            \"$ref\": \"#/$defs/positive_number\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"gen\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"structure to hold initial state of generator variables\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"pg\","]
#[doc = "          \"uid\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"down_time\": {"]
#[doc = "            \"description\": \"[h] if in service, zero, else time the unit has been out of service\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional generator initial condition parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"in_service_time\": {"]
#[doc = "            \"description\": \"[h] if in service, time the unit has been in service, zero otherwise\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"pg\": {"]
#[doc = "            \"description\": \"[MW or pu] initial active power injection\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"qg\": {"]
#[doc = "            \"description\": \"[MW or pu] initial reactive power injection\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"description\": \"uid of generator this record refers to\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"global_params\": {"]
#[doc = "      \"description\": \"structure to hold global parameters of temporal boundary\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"time_elapsed\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"time_elapsed\": {"]
#[doc = "          \"description\": \"[seconds] time elapsed since temporal_boundary conditions where present in the system\","]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"hvdc_p2p\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"structure to hold initial state of hvdc point-to-point line variables\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"pdc_fr\","]
#[doc = "          \"pdc_to\","]
#[doc = "          \"uid\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional hvdc point-to-point line initial condition parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"pdc_fr\": {"]
#[doc = "            \"description\": \"[MW or pu] initial active power entering hvdc line at from bus\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"pdc_to\": {"]
#[doc = "            \"description\": \"[MW or pu] initial active power entering hvdc line at to bus\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"qdc_fr\": {"]
#[doc = "            \"description\": \"[MVAr or pu] initial reactive power entering hvdc line at from bus\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"qdc_to\": {"]
#[doc = "            \"description\": \"[MVAr or pu] initial reactive power entering hvdc line at to bus\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"description\": \"uid of hvdc point-to-point this record refers to\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          },"]
#[doc = "          \"vm_dc_fr\": {"]
#[doc = "            \"description\": \"[kV or pu] initial dc side voltage at from converter\","]
#[doc = "            \"$ref\": \"#/$defs/positive_number\""]
#[doc = "          },"]
#[doc = "          \"vm_dc_to\": {"]
#[doc = "            \"description\": \"[kV or pu] initial dc side voltage at to converter\","]
#[doc = "            \"$ref\": \"#/$defs/positive_number\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"shunt\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"structure to hold initial state of shunt variables\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"num_steps\","]
#[doc = "          \"uid\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional shunt initial condition parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"num_steps\": {"]
#[doc = "            \"description\": \"[-] number of initial energized steps per section\","]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/nonnegative_integer\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"$ref\": \"#/$defs/nonnegative_integer\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"description\": \"uid of shunt this record refers to\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"storage\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"structure to hold initial state of storage variables\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"energy\","]
#[doc = "          \"uid\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"energy\": {"]
#[doc = "            \"description\": \"[MWh or pu*h] initial state of charge\","]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "          },"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional storage initial condition parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"ps\": {"]
#[doc = "            \"description\": \"[MW or pu] initial active power injection\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"qs\": {"]
#[doc = "            \"description\": \"[MW or pu] initial reactive power injection\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"description\": \"uid of storage this record refers to\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"switch\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"structure to hold initial state of switch variables\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"state\","]
#[doc = "          \"uid\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional switch initial condition parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"state\": {"]
#[doc = "            \"description\": \"[-] binary indicator of switch initial status; 0 => open, 1 => closed\","]
#[doc = "            \"$ref\": \"#/$defs/binary\""]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"description\": \"uid of switch this record refers to\","]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"transformer\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"structure to hold initial state of transformer variables\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"ta\","]
#[doc = "          \"tm\","]
#[doc = "          \"uid\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"ext\": {"]
#[doc = "            \"description\": \"additional transformer initial condition parameters currently not supported by CTM\""]
#[doc = "          },"]
#[doc = "          \"ta\": {"]
#[doc = "            \"description\": \"[deg] initial angle phase shift\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"tm\": {"]
#[doc = "            \"description\": \"[-] initial tap ratio\","]
#[doc = "            \"$ref\": \"#/$defs/positive_number\""]
#[doc = "          },"]
#[doc = "          \"uid\": {"]
#[doc = "            \"$ref\": \"#/$defs/uid\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataTemporalBoundary {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub bus: ::std::vec::Vec<CtmDataTemporalBoundaryBusItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub gen: ::std::vec::Vec<CtmDataTemporalBoundaryGenItem>,
    pub global_params: CtmDataTemporalBoundaryGlobalParams,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub hvdc_p2p: ::std::vec::Vec<CtmDataTemporalBoundaryHvdcP2pItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub shunt: ::std::vec::Vec<CtmDataTemporalBoundaryShuntItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub storage: ::std::vec::Vec<CtmDataTemporalBoundaryStorageItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub switch: ::std::vec::Vec<CtmDataTemporalBoundarySwitchItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub transformer: ::std::vec::Vec<CtmDataTemporalBoundaryTransformerItem>,
}
impl ::std::convert::From<&CtmDataTemporalBoundary> for CtmDataTemporalBoundary {
    fn from(value: &CtmDataTemporalBoundary) -> Self {
        value.clone()
    }
}
impl CtmDataTemporalBoundary {
    pub fn builder() -> builder::CtmDataTemporalBoundary {
        Default::default()
    }
}
#[doc = "structure to hold initial state of bus variables"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold initial state of bus variables\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"uid\","]
#[doc = "    \"va\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional bus initial condition parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"description\": \"uid of bus this record refers to\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"va\": {"]
#[doc = "      \"description\": \"[deg] initial voltage angle\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"vm\": {"]
#[doc = "      \"description\": \"[kV or pu] initial voltage magnitude\","]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataTemporalBoundaryBusItem {
    #[doc = "additional bus initial condition parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    #[doc = "uid of bus this record refers to"]
    pub uid: Uid,
    pub va: f64,
    #[doc = "[kV or pu] initial voltage magnitude"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub vm: ::std::option::Option<PositiveNumber>,
}
impl ::std::convert::From<&CtmDataTemporalBoundaryBusItem> for CtmDataTemporalBoundaryBusItem {
    fn from(value: &CtmDataTemporalBoundaryBusItem) -> Self {
        value.clone()
    }
}
impl CtmDataTemporalBoundaryBusItem {
    pub fn builder() -> builder::CtmDataTemporalBoundaryBusItem {
        Default::default()
    }
}
#[doc = "structure to hold initial state of generator variables"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold initial state of generator variables\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"pg\","]
#[doc = "    \"uid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"down_time\": {"]
#[doc = "      \"description\": \"[h] if in service, zero, else time the unit has been out of service\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional generator initial condition parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"in_service_time\": {"]
#[doc = "      \"description\": \"[h] if in service, time the unit has been in service, zero otherwise\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"pg\": {"]
#[doc = "      \"description\": \"[MW or pu] initial active power injection\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"qg\": {"]
#[doc = "      \"description\": \"[MW or pu] initial reactive power injection\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"description\": \"uid of generator this record refers to\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataTemporalBoundaryGenItem {
    #[doc = "[h] if in service, zero, else time the unit has been out of service"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub down_time: ::std::option::Option<NonnegativeNumber>,
    #[doc = "additional generator initial condition parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    #[doc = "[h] if in service, time the unit has been in service, zero otherwise"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub in_service_time: ::std::option::Option<NonnegativeNumber>,
    pub pg: f64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub qg: ::std::option::Option<f64>,
    #[doc = "uid of generator this record refers to"]
    pub uid: Uid,
}
impl ::std::convert::From<&CtmDataTemporalBoundaryGenItem> for CtmDataTemporalBoundaryGenItem {
    fn from(value: &CtmDataTemporalBoundaryGenItem) -> Self {
        value.clone()
    }
}
impl CtmDataTemporalBoundaryGenItem {
    pub fn builder() -> builder::CtmDataTemporalBoundaryGenItem {
        Default::default()
    }
}
#[doc = "structure to hold global parameters of temporal boundary"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold global parameters of temporal boundary\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"time_elapsed\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"time_elapsed\": {"]
#[doc = "      \"description\": \"[seconds] time elapsed since temporal_boundary conditions where present in the system\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataTemporalBoundaryGlobalParams {
    #[doc = "[seconds] time elapsed since temporal_boundary conditions where present in the system"]
    pub time_elapsed: NonnegativeNumber,
}
impl ::std::convert::From<&CtmDataTemporalBoundaryGlobalParams>
    for CtmDataTemporalBoundaryGlobalParams
{
    fn from(value: &CtmDataTemporalBoundaryGlobalParams) -> Self {
        value.clone()
    }
}
impl CtmDataTemporalBoundaryGlobalParams {
    pub fn builder() -> builder::CtmDataTemporalBoundaryGlobalParams {
        Default::default()
    }
}
#[doc = "structure to hold initial state of hvdc point-to-point line variables"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold initial state of hvdc point-to-point line variables\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"pdc_fr\","]
#[doc = "    \"pdc_to\","]
#[doc = "    \"uid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional hvdc point-to-point line initial condition parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"pdc_fr\": {"]
#[doc = "      \"description\": \"[MW or pu] initial active power entering hvdc line at from bus\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"pdc_to\": {"]
#[doc = "      \"description\": \"[MW or pu] initial active power entering hvdc line at to bus\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"qdc_fr\": {"]
#[doc = "      \"description\": \"[MVAr or pu] initial reactive power entering hvdc line at from bus\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"qdc_to\": {"]
#[doc = "      \"description\": \"[MVAr or pu] initial reactive power entering hvdc line at to bus\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"description\": \"uid of hvdc point-to-point this record refers to\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    },"]
#[doc = "    \"vm_dc_fr\": {"]
#[doc = "      \"description\": \"[kV or pu] initial dc side voltage at from converter\","]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    \"vm_dc_to\": {"]
#[doc = "      \"description\": \"[kV or pu] initial dc side voltage at to converter\","]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataTemporalBoundaryHvdcP2pItem {
    #[doc = "additional hvdc point-to-point line initial condition parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    pub pdc_fr: f64,
    pub pdc_to: f64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub qdc_fr: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub qdc_to: ::std::option::Option<f64>,
    #[doc = "uid of hvdc point-to-point this record refers to"]
    pub uid: Uid,
    #[doc = "[kV or pu] initial dc side voltage at from converter"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub vm_dc_fr: ::std::option::Option<PositiveNumber>,
    #[doc = "[kV or pu] initial dc side voltage at to converter"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub vm_dc_to: ::std::option::Option<PositiveNumber>,
}
impl ::std::convert::From<&CtmDataTemporalBoundaryHvdcP2pItem>
    for CtmDataTemporalBoundaryHvdcP2pItem
{
    fn from(value: &CtmDataTemporalBoundaryHvdcP2pItem) -> Self {
        value.clone()
    }
}
impl CtmDataTemporalBoundaryHvdcP2pItem {
    pub fn builder() -> builder::CtmDataTemporalBoundaryHvdcP2pItem {
        Default::default()
    }
}
#[doc = "structure to hold initial state of shunt variables"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold initial state of shunt variables\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"num_steps\","]
#[doc = "    \"uid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional shunt initial condition parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"num_steps\": {"]
#[doc = "      \"description\": \"[-] number of initial energized steps per section\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/nonnegative_integer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/nonnegative_integer\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"description\": \"uid of shunt this record refers to\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataTemporalBoundaryShuntItem {
    #[doc = "additional shunt initial condition parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    #[doc = "[-] number of initial energized steps per section"]
    pub num_steps: CtmDataTemporalBoundaryShuntItemNumSteps,
    #[doc = "uid of shunt this record refers to"]
    pub uid: Uid,
}
impl ::std::convert::From<&CtmDataTemporalBoundaryShuntItem> for CtmDataTemporalBoundaryShuntItem {
    fn from(value: &CtmDataTemporalBoundaryShuntItem) -> Self {
        value.clone()
    }
}
impl CtmDataTemporalBoundaryShuntItem {
    pub fn builder() -> builder::CtmDataTemporalBoundaryShuntItem {
        Default::default()
    }
}
#[doc = "[-] number of initial energized steps per section"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[-] number of initial energized steps per section\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_integer\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/nonnegative_integer\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataTemporalBoundaryShuntItemNumSteps {
    Variant0(NonnegativeInteger),
    Variant1(::std::vec::Vec<NonnegativeInteger>),
}
impl ::std::convert::From<&Self> for CtmDataTemporalBoundaryShuntItemNumSteps {
    fn from(value: &CtmDataTemporalBoundaryShuntItemNumSteps) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonnegativeInteger> for CtmDataTemporalBoundaryShuntItemNumSteps {
    fn from(value: NonnegativeInteger) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<NonnegativeInteger>>
    for CtmDataTemporalBoundaryShuntItemNumSteps
{
    fn from(value: ::std::vec::Vec<NonnegativeInteger>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "structure to hold initial state of storage variables"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold initial state of storage variables\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"energy\","]
#[doc = "    \"uid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"energy\": {"]
#[doc = "      \"description\": \"[MWh or pu*h] initial state of charge\","]
#[doc = "      \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "    },"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional storage initial condition parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"ps\": {"]
#[doc = "      \"description\": \"[MW or pu] initial active power injection\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"qs\": {"]
#[doc = "      \"description\": \"[MW or pu] initial reactive power injection\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"description\": \"uid of storage this record refers to\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataTemporalBoundaryStorageItem {
    #[doc = "[MWh or pu*h] initial state of charge"]
    pub energy: NonnegativeNumber,
    #[doc = "additional storage initial condition parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ps: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub qs: ::std::option::Option<f64>,
    #[doc = "uid of storage this record refers to"]
    pub uid: Uid,
}
impl ::std::convert::From<&CtmDataTemporalBoundaryStorageItem>
    for CtmDataTemporalBoundaryStorageItem
{
    fn from(value: &CtmDataTemporalBoundaryStorageItem) -> Self {
        value.clone()
    }
}
impl CtmDataTemporalBoundaryStorageItem {
    pub fn builder() -> builder::CtmDataTemporalBoundaryStorageItem {
        Default::default()
    }
}
#[doc = "structure to hold initial state of switch variables"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold initial state of switch variables\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"state\","]
#[doc = "    \"uid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional switch initial condition parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"state\": {"]
#[doc = "      \"description\": \"[-] binary indicator of switch initial status; 0 => open, 1 => closed\","]
#[doc = "      \"$ref\": \"#/$defs/binary\""]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"description\": \"uid of switch this record refers to\","]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataTemporalBoundarySwitchItem {
    #[doc = "additional switch initial condition parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    #[doc = "[-] binary indicator of switch initial status; 0 => open, 1 => closed"]
    pub state: Binary,
    #[doc = "uid of switch this record refers to"]
    pub uid: Uid,
}
impl ::std::convert::From<&CtmDataTemporalBoundarySwitchItem>
    for CtmDataTemporalBoundarySwitchItem
{
    fn from(value: &CtmDataTemporalBoundarySwitchItem) -> Self {
        value.clone()
    }
}
impl CtmDataTemporalBoundarySwitchItem {
    pub fn builder() -> builder::CtmDataTemporalBoundarySwitchItem {
        Default::default()
    }
}
#[doc = "structure to hold initial state of transformer variables"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold initial state of transformer variables\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"ta\","]
#[doc = "    \"tm\","]
#[doc = "    \"uid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional transformer initial condition parameters currently not supported by CTM\""]
#[doc = "    },"]
#[doc = "    \"ta\": {"]
#[doc = "      \"description\": \"[deg] initial angle phase shift\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"tm\": {"]
#[doc = "      \"description\": \"[-] initial tap ratio\","]
#[doc = "      \"$ref\": \"#/$defs/positive_number\""]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"$ref\": \"#/$defs/uid\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataTemporalBoundaryTransformerItem {
    #[doc = "additional transformer initial condition parameters currently not supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext: ::std::option::Option<::serde_json::Value>,
    pub ta: f64,
    #[doc = "[-] initial tap ratio"]
    pub tm: PositiveNumber,
    pub uid: Uid,
}
impl ::std::convert::From<&CtmDataTemporalBoundaryTransformerItem>
    for CtmDataTemporalBoundaryTransformerItem
{
    fn from(value: &CtmDataTemporalBoundaryTransformerItem) -> Self {
        value.clone()
    }
}
impl CtmDataTemporalBoundaryTransformerItem {
    pub fn builder() -> builder::CtmDataTemporalBoundaryTransformerItem {
        Default::default()
    }
}
#[doc = "structure to contain all time variant data of the system/case. All time series are synchronized to the same timestamps, which should should be stored using Unix time. Structure is quasi-tabular, with uid, name, path_to_file, values, and ext being arrays in the same order of said field. This is done in order to allow for better compression (e.g., using HDF5) for the values field."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to contain all time variant data of the system/case. All time series are synchronized to the same timestamps, which should should be stored using Unix time. Structure is quasi-tabular, with uid, name, path_to_file, values, and ext being arrays in the same order of said field. This is done in order to allow for better compression (e.g., using HDF5) for the values field.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"uid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ext\": {"]
#[doc = "      \"description\": \"additional time series information not currently supported by CTM\","]
#[doc = "      \"type\": \"array\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"array of names of time series\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"path_to_file\": {"]
#[doc = "      \"description\": \"path to file containing all time series information or a separate path for each time series\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"description\": \"[seconds] seconds since epoch (Unix time) for each instant for which time series values are provided\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/nonnegative_number\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"description\": \"array of uids of time series\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/uid\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"values\": {"]
#[doc = "      \"description\": \"array of time series values\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"time series values for a particular time series\","]
#[doc = "        \"type\": \"array\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CtmDataTimeSeriesData {
    #[doc = "additional time series information not currently supported by CTM"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub ext: ::std::vec::Vec<::serde_json::Value>,
    #[doc = "array of names of time series"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub name: ::std::vec::Vec<::std::string::String>,
    #[doc = "path to file containing all time series information or a separate path for each time series"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub path_to_file: ::std::option::Option<CtmDataTimeSeriesDataPathToFile>,
    #[doc = "[seconds] seconds since epoch (Unix time) for each instant for which time series values are provided"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub timestamp: ::std::vec::Vec<NonnegativeNumber>,
    #[doc = "array of uids of time series"]
    pub uid: ::std::vec::Vec<Uid>,
    #[doc = "array of time series values"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub values: ::std::vec::Vec<::std::vec::Vec<::serde_json::Value>>,
}
impl ::std::convert::From<&CtmDataTimeSeriesData> for CtmDataTimeSeriesData {
    fn from(value: &CtmDataTimeSeriesData) -> Self {
        value.clone()
    }
}
impl CtmDataTimeSeriesData {
    pub fn builder() -> builder::CtmDataTimeSeriesData {
        Default::default()
    }
}
#[doc = "path to file containing all time series information or a separate path for each time series"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"path to file containing all time series information or a separate path for each time series\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CtmDataTimeSeriesDataPathToFile {
    Variant0(::std::string::String),
    Variant1(::std::vec::Vec<::std::string::String>),
}
impl ::std::convert::From<&Self> for CtmDataTimeSeriesDataPathToFile {
    fn from(value: &CtmDataTimeSeriesDataPathToFile) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>>
    for CtmDataTimeSeriesDataPathToFile
{
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "nonnegative integer number"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"nonnegative integer number\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"minimum\": 0.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct NonnegativeInteger(pub u64);
impl ::std::ops::Deref for NonnegativeInteger {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.0
    }
}
impl ::std::convert::From<NonnegativeInteger> for u64 {
    fn from(value: NonnegativeInteger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NonnegativeInteger> for NonnegativeInteger {
    fn from(value: &NonnegativeInteger) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<u64> for NonnegativeInteger {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for NonnegativeInteger {
    type Err = <u64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for NonnegativeInteger {
    type Error = <u64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for NonnegativeInteger {
    type Error = <u64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for NonnegativeInteger {
    type Error = <u64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for NonnegativeInteger {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`NonnegativeNumber`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"nonnegative real number\","]
#[doc = "  \"type\": \"number\","]
#[doc = "  \"minimum\": 0.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct NonnegativeNumber(pub f64);
impl ::std::ops::Deref for NonnegativeNumber {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl ::std::convert::From<NonnegativeNumber> for f64 {
    fn from(value: NonnegativeNumber) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NonnegativeNumber> for NonnegativeNumber {
    fn from(value: &NonnegativeNumber) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for NonnegativeNumber {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for NonnegativeNumber {
    type Err = <f64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for NonnegativeNumber {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for NonnegativeNumber {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for NonnegativeNumber {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for NonnegativeNumber {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "positive integer number"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"positive integer number\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"exclusiveMinimum\": 0.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PositiveInteger(pub ::std::num::NonZeroU64);
impl ::std::ops::Deref for PositiveInteger {
    type Target = ::std::num::NonZeroU64;
    fn deref(&self) -> &::std::num::NonZeroU64 {
        &self.0
    }
}
impl ::std::convert::From<PositiveInteger> for ::std::num::NonZeroU64 {
    fn from(value: PositiveInteger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PositiveInteger> for PositiveInteger {
    fn from(value: &PositiveInteger) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::num::NonZeroU64> for PositiveInteger {
    fn from(value: ::std::num::NonZeroU64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for PositiveInteger {
    type Err = <::std::num::NonZeroU64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for PositiveInteger {
    type Error = <::std::num::NonZeroU64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for PositiveInteger {
    type Error = <::std::num::NonZeroU64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for PositiveInteger {
    type Error = <::std::num::NonZeroU64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for PositiveInteger {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`PositiveNumber`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"positive real number\","]
#[doc = "  \"type\": \"number\","]
#[doc = "  \"exclusiveMinimum\": 0.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PositiveNumber(pub f64);
impl ::std::ops::Deref for PositiveNumber {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl ::std::convert::From<PositiveNumber> for f64 {
    fn from(value: PositiveNumber) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PositiveNumber> for PositiveNumber {
    fn from(value: &PositiveNumber) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for PositiveNumber {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for PositiveNumber {
    type Err = <f64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for PositiveNumber {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for PositiveNumber {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for PositiveNumber {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for PositiveNumber {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "binary indicator of whether component should be included or omitted; 1=>included, 0=>omitted"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"binary indicator of whether component should be included or omitted; 1=>included, 0=>omitted\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"maximum\": 1.0,"]
#[doc = "  \"minimum\": 0.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Status(pub i64);
impl ::std::ops::Deref for Status {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<Status> for i64 {
    fn from(value: Status) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Status> for Status {
    fn from(value: &Status) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for Status {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Status {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Status {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for Status {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Status {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for Status {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "structure to hold a reference (possibly, to be scaled) to a time series"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"structure to hold a reference (possibly, to be scaled) to a time series\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"scale_factor\","]
#[doc = "    \"uid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"scale_factor\": {"]
#[doc = "      \"description\": \"[-] scale factor to be applied to the pointed-to time series to obtain this field's values\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"uid\": {"]
#[doc = "      \"description\": \"uid of time series (in time_series_data) this reference points to\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TimeSeriesReference {
    pub scale_factor: f64,
    #[doc = "uid of time series (in time_series_data) this reference points to"]
    pub uid: TimeSeriesReferenceUid,
}
impl ::std::convert::From<&TimeSeriesReference> for TimeSeriesReference {
    fn from(value: &TimeSeriesReference) -> Self {
        value.clone()
    }
}
impl TimeSeriesReference {
    pub fn builder() -> builder::TimeSeriesReference {
        Default::default()
    }
}
#[doc = "uid of time series (in time_series_data) this reference points to"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"uid of time series (in time_series_data) this reference points to\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TimeSeriesReferenceUid {
    Variant0(u64),
    Variant1(::std::string::String),
}
impl ::std::convert::From<&Self> for TimeSeriesReferenceUid {
    fn from(value: &TimeSeriesReferenceUid) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TimeSeriesReferenceUid {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for TimeSeriesReferenceUid {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TimeSeriesReferenceUid {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TimeSeriesReferenceUid {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for TimeSeriesReferenceUid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<u64> for TimeSeriesReferenceUid {
    fn from(value: u64) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "object's Unique IDentifier"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"object's Unique IDentifier\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Uid {
    Variant0(u64),
    Variant1(::std::string::String),
}
impl ::std::convert::From<&Self> for Uid {
    fn from(value: &Uid) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Uid {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for Uid {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Uid {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Uid {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for Uid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<u64> for Uid {
    fn from(value: u64) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "pairs of data points saved as two vectors (of the same length)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"pairs of data points saved as two vectors (of the same length)\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"x\","]
#[doc = "    \"y\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"x\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"y\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct XyPairs {
    pub x: ::std::vec::Vec<f64>,
    pub y: ::std::vec::Vec<f64>,
}
impl ::std::convert::From<&XyPairs> for XyPairs {
    fn from(value: &XyPairs) -> Self {
        value.clone()
    }
}
impl XyPairs {
    pub fn builder() -> builder::XyPairs {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CtmData {
        ctm_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        network: ::std::result::Result<super::CtmDataNetwork, ::std::string::String>,
        temporal_boundary:
            ::std::result::Result<super::CtmDataTemporalBoundary, ::std::string::String>,
        time_series_data: ::std::result::Result<
            ::std::option::Option<super::CtmDataTimeSeriesData>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CtmData {
        fn default() -> Self {
            Self {
                ctm_version: Err("no value supplied for ctm_version".to_string()),
                network: Err("no value supplied for network".to_string()),
                temporal_boundary: Err("no value supplied for temporal_boundary".to_string()),
                time_series_data: Ok(Default::default()),
            }
        }
    }
    impl CtmData {
        pub fn ctm_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ctm_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ctm_version: {}", e));
            self
        }
        pub fn network<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CtmDataNetwork>,
            T::Error: ::std::fmt::Display,
        {
            self.network = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for network: {}", e));
            self
        }
        pub fn temporal_boundary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CtmDataTemporalBoundary>,
            T::Error: ::std::fmt::Display,
        {
            self.temporal_boundary = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for temporal_boundary: {}",
                    e
                )
            });
            self
        }
        pub fn time_series_data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataTimeSeriesData>>,
            T::Error: ::std::fmt::Display,
        {
            self.time_series_data = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for time_series_data: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<CtmData> for super::CtmData {
        type Error = super::error::ConversionError;
        fn try_from(value: CtmData) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ctm_version: value.ctm_version?,
                network: value.network?,
                temporal_boundary: value.temporal_boundary?,
                time_series_data: value.time_series_data?,
            })
        }
    }
    impl ::std::convert::From<super::CtmData> for CtmData {
        fn from(value: super::CtmData) -> Self {
            Self {
                ctm_version: Ok(value.ctm_version),
                network: Ok(value.network),
                temporal_boundary: Ok(value.temporal_boundary),
                time_series_data: Ok(value.time_series_data),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataNetwork {
        ac_line: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataNetworkAcLineItem>,
            ::std::string::String,
        >,
        area: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataNetworkAreaItem>,
            ::std::string::String,
        >,
        bus: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataNetworkBusItem>,
            ::std::string::String,
        >,
        gen: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataNetworkGenItem>,
            ::std::string::String,
        >,
        global_params:
            ::std::result::Result<super::CtmDataNetworkGlobalParams, ::std::string::String>,
        hvdc_p2p: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataNetworkHvdcP2pItem>,
            ::std::string::String,
        >,
        load: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataNetworkLoadItem>,
            ::std::string::String,
        >,
        reserve: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataNetworkReserveItem>,
            ::std::string::String,
        >,
        shunt: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataNetworkShuntItem>,
            ::std::string::String,
        >,
        storage: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataNetworkStorageItem>,
            ::std::string::String,
        >,
        switch: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataNetworkSwitchItem>,
            ::std::string::String,
        >,
        transformer: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataNetworkTransformerItem>,
            ::std::string::String,
        >,
        zone: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataNetworkZoneItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CtmDataNetwork {
        fn default() -> Self {
            Self {
                ac_line: Ok(Default::default()),
                area: Err("no value supplied for area".to_string()),
                bus: Err("no value supplied for bus".to_string()),
                gen: Err("no value supplied for gen".to_string()),
                global_params: Err("no value supplied for global_params".to_string()),
                hvdc_p2p: Ok(Default::default()),
                load: Err("no value supplied for load".to_string()),
                reserve: Ok(Default::default()),
                shunt: Ok(Default::default()),
                storage: Ok(Default::default()),
                switch: Ok(Default::default()),
                transformer: Ok(Default::default()),
                zone: Ok(Default::default()),
            }
        }
    }
    impl CtmDataNetwork {
        pub fn ac_line<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CtmDataNetworkAcLineItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.ac_line = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ac_line: {}", e));
            self
        }
        pub fn area<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CtmDataNetworkAreaItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.area = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for area: {}", e));
            self
        }
        pub fn bus<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CtmDataNetworkBusItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.bus = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bus: {}", e));
            self
        }
        pub fn gen<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CtmDataNetworkGenItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.gen = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gen: {}", e));
            self
        }
        pub fn global_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CtmDataNetworkGlobalParams>,
            T::Error: ::std::fmt::Display,
        {
            self.global_params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for global_params: {}", e));
            self
        }
        pub fn hvdc_p2p<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CtmDataNetworkHvdcP2pItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.hvdc_p2p = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hvdc_p2p: {}", e));
            self
        }
        pub fn load<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CtmDataNetworkLoadItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.load = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for load: {}", e));
            self
        }
        pub fn reserve<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CtmDataNetworkReserveItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.reserve = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reserve: {}", e));
            self
        }
        pub fn shunt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CtmDataNetworkShuntItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.shunt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shunt: {}", e));
            self
        }
        pub fn storage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CtmDataNetworkStorageItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.storage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for storage: {}", e));
            self
        }
        pub fn switch<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CtmDataNetworkSwitchItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.switch = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for switch: {}", e));
            self
        }
        pub fn transformer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CtmDataNetworkTransformerItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.transformer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for transformer: {}", e));
            self
        }
        pub fn zone<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CtmDataNetworkZoneItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.zone = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for zone: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataNetwork> for super::CtmDataNetwork {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataNetwork,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ac_line: value.ac_line?,
                area: value.area?,
                bus: value.bus?,
                gen: value.gen?,
                global_params: value.global_params?,
                hvdc_p2p: value.hvdc_p2p?,
                load: value.load?,
                reserve: value.reserve?,
                shunt: value.shunt?,
                storage: value.storage?,
                switch: value.switch?,
                transformer: value.transformer?,
                zone: value.zone?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataNetwork> for CtmDataNetwork {
        fn from(value: super::CtmDataNetwork) -> Self {
            Self {
                ac_line: Ok(value.ac_line),
                area: Ok(value.area),
                bus: Ok(value.bus),
                gen: Ok(value.gen),
                global_params: Ok(value.global_params),
                hvdc_p2p: Ok(value.hvdc_p2p),
                load: Ok(value.load),
                reserve: Ok(value.reserve),
                shunt: Ok(value.shunt),
                storage: Ok(value.storage),
                switch: Ok(value.switch),
                transformer: Ok(value.transformer),
                zone: Ok(value.zone),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataNetworkAcLineItem {
        b_fr: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        b_to: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        bus_fr: ::std::result::Result<super::Uid, ::std::string::String>,
        bus_to: ::std::result::Result<super::Uid, ::std::string::String>,
        cm_ub_a: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkAcLineItemCmUbA>,
            ::std::string::String,
        >,
        cm_ub_b: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkAcLineItemCmUbB>,
            ::std::string::String,
        >,
        cm_ub_c: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkAcLineItemCmUbC>,
            ::std::string::String,
        >,
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        g_fr: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        g_to: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        nominal_mva: ::std::result::Result<
            ::std::option::Option<super::PositiveNumber>,
            ::std::string::String,
        >,
        persistent_outage_duration: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        persistent_outage_rate: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        r: ::std::result::Result<f64, ::std::string::String>,
        sm_ub_a: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkAcLineItemSmUbA>,
            ::std::string::String,
        >,
        sm_ub_b: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkAcLineItemSmUbB>,
            ::std::string::String,
        >,
        sm_ub_c: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkAcLineItemSmUbC>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::Status, ::std::string::String>,
        transient_outage_rate: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
        vad_lb: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        vad_ub: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        x: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for CtmDataNetworkAcLineItem {
        fn default() -> Self {
            Self {
                b_fr: Ok(Default::default()),
                b_to: Ok(Default::default()),
                bus_fr: Err("no value supplied for bus_fr".to_string()),
                bus_to: Err("no value supplied for bus_to".to_string()),
                cm_ub_a: Ok(Default::default()),
                cm_ub_b: Ok(Default::default()),
                cm_ub_c: Ok(Default::default()),
                ext: Ok(Default::default()),
                g_fr: Ok(Default::default()),
                g_to: Ok(Default::default()),
                name: Ok(Default::default()),
                nominal_mva: Ok(Default::default()),
                persistent_outage_duration: Ok(Default::default()),
                persistent_outage_rate: Ok(Default::default()),
                r: Err("no value supplied for r".to_string()),
                sm_ub_a: Ok(Default::default()),
                sm_ub_b: Ok(Default::default()),
                sm_ub_c: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
                transient_outage_rate: Ok(Default::default()),
                uid: Err("no value supplied for uid".to_string()),
                vad_lb: Ok(Default::default()),
                vad_ub: Ok(Default::default()),
                x: Err("no value supplied for x".to_string()),
            }
        }
    }
    impl CtmDataNetworkAcLineItem {
        pub fn b_fr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.b_fr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for b_fr: {}", e));
            self
        }
        pub fn b_to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.b_to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for b_to: {}", e));
            self
        }
        pub fn bus_fr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.bus_fr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bus_fr: {}", e));
            self
        }
        pub fn bus_to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.bus_to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bus_to: {}", e));
            self
        }
        pub fn cm_ub_a<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkAcLineItemCmUbA>>,
            T::Error: ::std::fmt::Display,
        {
            self.cm_ub_a = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cm_ub_a: {}", e));
            self
        }
        pub fn cm_ub_b<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkAcLineItemCmUbB>>,
            T::Error: ::std::fmt::Display,
        {
            self.cm_ub_b = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cm_ub_b: {}", e));
            self
        }
        pub fn cm_ub_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkAcLineItemCmUbC>>,
            T::Error: ::std::fmt::Display,
        {
            self.cm_ub_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cm_ub_c: {}", e));
            self
        }
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn g_fr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.g_fr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for g_fr: {}", e));
            self
        }
        pub fn g_to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.g_to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for g_to: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn nominal_mva<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PositiveNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.nominal_mva = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nominal_mva: {}", e));
            self
        }
        pub fn persistent_outage_duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.persistent_outage_duration = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for persistent_outage_duration: {}",
                    e
                )
            });
            self
        }
        pub fn persistent_outage_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.persistent_outage_rate = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for persistent_outage_rate: {}",
                    e
                )
            });
            self
        }
        pub fn r<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.r = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for r: {}", e));
            self
        }
        pub fn sm_ub_a<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkAcLineItemSmUbA>>,
            T::Error: ::std::fmt::Display,
        {
            self.sm_ub_a = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sm_ub_a: {}", e));
            self
        }
        pub fn sm_ub_b<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkAcLineItemSmUbB>>,
            T::Error: ::std::fmt::Display,
        {
            self.sm_ub_b = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sm_ub_b: {}", e));
            self
        }
        pub fn sm_ub_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkAcLineItemSmUbC>>,
            T::Error: ::std::fmt::Display,
        {
            self.sm_ub_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sm_ub_c: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Status>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn transient_outage_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.transient_outage_rate = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for transient_outage_rate: {}",
                    e
                )
            });
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
        pub fn vad_lb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.vad_lb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vad_lb: {}", e));
            self
        }
        pub fn vad_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.vad_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vad_ub: {}", e));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataNetworkAcLineItem> for super::CtmDataNetworkAcLineItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataNetworkAcLineItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                b_fr: value.b_fr?,
                b_to: value.b_to?,
                bus_fr: value.bus_fr?,
                bus_to: value.bus_to?,
                cm_ub_a: value.cm_ub_a?,
                cm_ub_b: value.cm_ub_b?,
                cm_ub_c: value.cm_ub_c?,
                ext: value.ext?,
                g_fr: value.g_fr?,
                g_to: value.g_to?,
                name: value.name?,
                nominal_mva: value.nominal_mva?,
                persistent_outage_duration: value.persistent_outage_duration?,
                persistent_outage_rate: value.persistent_outage_rate?,
                r: value.r?,
                sm_ub_a: value.sm_ub_a?,
                sm_ub_b: value.sm_ub_b?,
                sm_ub_c: value.sm_ub_c?,
                status: value.status?,
                transient_outage_rate: value.transient_outage_rate?,
                uid: value.uid?,
                vad_lb: value.vad_lb?,
                vad_ub: value.vad_ub?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataNetworkAcLineItem> for CtmDataNetworkAcLineItem {
        fn from(value: super::CtmDataNetworkAcLineItem) -> Self {
            Self {
                b_fr: Ok(value.b_fr),
                b_to: Ok(value.b_to),
                bus_fr: Ok(value.bus_fr),
                bus_to: Ok(value.bus_to),
                cm_ub_a: Ok(value.cm_ub_a),
                cm_ub_b: Ok(value.cm_ub_b),
                cm_ub_c: Ok(value.cm_ub_c),
                ext: Ok(value.ext),
                g_fr: Ok(value.g_fr),
                g_to: Ok(value.g_to),
                name: Ok(value.name),
                nominal_mva: Ok(value.nominal_mva),
                persistent_outage_duration: Ok(value.persistent_outage_duration),
                persistent_outage_rate: Ok(value.persistent_outage_rate),
                r: Ok(value.r),
                sm_ub_a: Ok(value.sm_ub_a),
                sm_ub_b: Ok(value.sm_ub_b),
                sm_ub_c: Ok(value.sm_ub_c),
                status: Ok(value.status),
                transient_outage_rate: Ok(value.transient_outage_rate),
                uid: Ok(value.uid),
                vad_lb: Ok(value.vad_lb),
                vad_ub: Ok(value.vad_ub),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataNetworkAreaItem {
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::Status, ::std::string::String>,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
    }
    impl ::std::default::Default for CtmDataNetworkAreaItem {
        fn default() -> Self {
            Self {
                ext: Ok(Default::default()),
                name: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
                uid: Err("no value supplied for uid".to_string()),
            }
        }
    }
    impl CtmDataNetworkAreaItem {
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Status>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataNetworkAreaItem> for super::CtmDataNetworkAreaItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataNetworkAreaItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ext: value.ext?,
                name: value.name?,
                status: value.status?,
                uid: value.uid?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataNetworkAreaItem> for CtmDataNetworkAreaItem {
        fn from(value: super::CtmDataNetworkAreaItem) -> Self {
            Self {
                ext: Ok(value.ext),
                name: Ok(value.name),
                status: Ok(value.status),
                uid: Ok(value.uid),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataNetworkBusItem {
        area: ::std::result::Result<::std::option::Option<super::Uid>, ::std::string::String>,
        base_kv: ::std::result::Result<super::PositiveNumber, ::std::string::String>,
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::Status, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkBusItemType>,
            ::std::string::String,
        >,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
        vm_lb: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkBusItemVmLb>,
            ::std::string::String,
        >,
        vm_ub: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkBusItemVmUb>,
            ::std::string::String,
        >,
        zone: ::std::result::Result<::std::option::Option<super::Uid>, ::std::string::String>,
    }
    impl ::std::default::Default for CtmDataNetworkBusItem {
        fn default() -> Self {
            Self {
                area: Ok(Default::default()),
                base_kv: Err("no value supplied for base_kv".to_string()),
                ext: Ok(Default::default()),
                name: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
                type_: Ok(Default::default()),
                uid: Err("no value supplied for uid".to_string()),
                vm_lb: Ok(Default::default()),
                vm_ub: Ok(Default::default()),
                zone: Ok(Default::default()),
            }
        }
    }
    impl CtmDataNetworkBusItem {
        pub fn area<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Uid>>,
            T::Error: ::std::fmt::Display,
        {
            self.area = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for area: {}", e));
            self
        }
        pub fn base_kv<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PositiveNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.base_kv = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base_kv: {}", e));
            self
        }
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Status>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkBusItemType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
        pub fn vm_lb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkBusItemVmLb>>,
            T::Error: ::std::fmt::Display,
        {
            self.vm_lb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vm_lb: {}", e));
            self
        }
        pub fn vm_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkBusItemVmUb>>,
            T::Error: ::std::fmt::Display,
        {
            self.vm_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vm_ub: {}", e));
            self
        }
        pub fn zone<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Uid>>,
            T::Error: ::std::fmt::Display,
        {
            self.zone = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for zone: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataNetworkBusItem> for super::CtmDataNetworkBusItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataNetworkBusItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                area: value.area?,
                base_kv: value.base_kv?,
                ext: value.ext?,
                name: value.name?,
                status: value.status?,
                type_: value.type_?,
                uid: value.uid?,
                vm_lb: value.vm_lb?,
                vm_ub: value.vm_ub?,
                zone: value.zone?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataNetworkBusItem> for CtmDataNetworkBusItem {
        fn from(value: super::CtmDataNetworkBusItem) -> Self {
            Self {
                area: Ok(value.area),
                base_kv: Ok(value.base_kv),
                ext: Ok(value.ext),
                name: Ok(value.name),
                status: Ok(value.status),
                type_: Ok(value.type_),
                uid: Ok(value.uid),
                vm_lb: Ok(value.vm_lb),
                vm_ub: Ok(value.vm_ub),
                zone: Ok(value.zone),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataNetworkGenItem {
        bus: ::std::result::Result<super::Uid, ::std::string::String>,
        cost_pg_model: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkGenItemCostPgModel>,
            ::std::string::String,
        >,
        cost_pg_parameters: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkGenItemCostPgParameters>,
            ::std::string::String,
        >,
        down_time_lb: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        forced_outage_rate:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        in_service_time_lb: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        in_service_time_ub: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        mean_time_to_failure: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        mean_time_to_repair: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        nominal_mva: ::std::result::Result<
            ::std::option::Option<super::PositiveNumber>,
            ::std::string::String,
        >,
        pg_delta_lb: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        pg_delta_ub: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        pg_lb: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkGenItemPgLb>,
            ::std::string::String,
        >,
        pg_ub: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkGenItemPgUb>,
            ::std::string::String,
        >,
        primary_source: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkGenItemPrimarySource>,
            ::std::string::String,
        >,
        primary_source_subtype: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkGenItemPrimarySourceSubtype>,
            ::std::string::String,
        >,
        qg_lb: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkGenItemQgLb>,
            ::std::string::String,
        >,
        qg_ub: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkGenItemQgUb>,
            ::std::string::String,
        >,
        scheduled_maintenance_rate:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        service_required: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkGenItemServiceRequired>,
            ::std::string::String,
        >,
        shutdown_cost: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkGenItemShutdownCost>,
            ::std::string::String,
        >,
        startup_cost_cold: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkGenItemStartupCostCold>,
            ::std::string::String,
        >,
        startup_cost_hot: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkGenItemStartupCostHot>,
            ::std::string::String,
        >,
        startup_cost_warm: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkGenItemStartupCostWarm>,
            ::std::string::String,
        >,
        startup_time_hot: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        startup_time_warm: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::Status, ::std::string::String>,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
        vm_setpoint: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkGenItemVmSetpoint>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CtmDataNetworkGenItem {
        fn default() -> Self {
            Self {
                bus: Err("no value supplied for bus".to_string()),
                cost_pg_model: Ok(Default::default()),
                cost_pg_parameters: Ok(Default::default()),
                down_time_lb: Ok(Default::default()),
                ext: Ok(Default::default()),
                forced_outage_rate: Ok(Default::default()),
                in_service_time_lb: Ok(Default::default()),
                in_service_time_ub: Ok(Default::default()),
                mean_time_to_failure: Ok(Default::default()),
                mean_time_to_repair: Ok(Default::default()),
                name: Ok(Default::default()),
                nominal_mva: Ok(Default::default()),
                pg_delta_lb: Ok(Default::default()),
                pg_delta_ub: Ok(Default::default()),
                pg_lb: Ok(Default::default()),
                pg_ub: Ok(Default::default()),
                primary_source: Ok(Default::default()),
                primary_source_subtype: Ok(Default::default()),
                qg_lb: Ok(Default::default()),
                qg_ub: Ok(Default::default()),
                scheduled_maintenance_rate: Ok(Default::default()),
                service_required: Ok(Default::default()),
                shutdown_cost: Ok(Default::default()),
                startup_cost_cold: Ok(Default::default()),
                startup_cost_hot: Ok(Default::default()),
                startup_cost_warm: Ok(Default::default()),
                startup_time_hot: Ok(Default::default()),
                startup_time_warm: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
                uid: Err("no value supplied for uid".to_string()),
                vm_setpoint: Ok(Default::default()),
            }
        }
    }
    impl CtmDataNetworkGenItem {
        pub fn bus<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.bus = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bus: {}", e));
            self
        }
        pub fn cost_pg_model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkGenItemCostPgModel>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.cost_pg_model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cost_pg_model: {}", e));
            self
        }
        pub fn cost_pg_parameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkGenItemCostPgParameters>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.cost_pg_parameters = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for cost_pg_parameters: {}",
                    e
                )
            });
            self
        }
        pub fn down_time_lb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.down_time_lb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for down_time_lb: {}", e));
            self
        }
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn forced_outage_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.forced_outage_rate = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for forced_outage_rate: {}",
                    e
                )
            });
            self
        }
        pub fn in_service_time_lb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.in_service_time_lb = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for in_service_time_lb: {}",
                    e
                )
            });
            self
        }
        pub fn in_service_time_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.in_service_time_ub = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for in_service_time_ub: {}",
                    e
                )
            });
            self
        }
        pub fn mean_time_to_failure<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.mean_time_to_failure = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for mean_time_to_failure: {}",
                    e
                )
            });
            self
        }
        pub fn mean_time_to_repair<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.mean_time_to_repair = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for mean_time_to_repair: {}",
                    e
                )
            });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn nominal_mva<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PositiveNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.nominal_mva = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nominal_mva: {}", e));
            self
        }
        pub fn pg_delta_lb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.pg_delta_lb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pg_delta_lb: {}", e));
            self
        }
        pub fn pg_delta_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.pg_delta_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pg_delta_ub: {}", e));
            self
        }
        pub fn pg_lb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkGenItemPgLb>>,
            T::Error: ::std::fmt::Display,
        {
            self.pg_lb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pg_lb: {}", e));
            self
        }
        pub fn pg_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkGenItemPgUb>>,
            T::Error: ::std::fmt::Display,
        {
            self.pg_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pg_ub: {}", e));
            self
        }
        pub fn primary_source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkGenItemPrimarySource>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.primary_source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for primary_source: {}", e));
            self
        }
        pub fn primary_source_subtype<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkGenItemPrimarySourceSubtype>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.primary_source_subtype = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for primary_source_subtype: {}",
                    e
                )
            });
            self
        }
        pub fn qg_lb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkGenItemQgLb>>,
            T::Error: ::std::fmt::Display,
        {
            self.qg_lb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for qg_lb: {}", e));
            self
        }
        pub fn qg_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkGenItemQgUb>>,
            T::Error: ::std::fmt::Display,
        {
            self.qg_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for qg_ub: {}", e));
            self
        }
        pub fn scheduled_maintenance_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.scheduled_maintenance_rate = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for scheduled_maintenance_rate: {}",
                    e
                )
            });
            self
        }
        pub fn service_required<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkGenItemServiceRequired>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.service_required = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for service_required: {}",
                    e
                )
            });
            self
        }
        pub fn shutdown_cost<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkGenItemShutdownCost>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.shutdown_cost = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shutdown_cost: {}", e));
            self
        }
        pub fn startup_cost_cold<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkGenItemStartupCostCold>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.startup_cost_cold = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for startup_cost_cold: {}",
                    e
                )
            });
            self
        }
        pub fn startup_cost_hot<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkGenItemStartupCostHot>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.startup_cost_hot = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for startup_cost_hot: {}",
                    e
                )
            });
            self
        }
        pub fn startup_cost_warm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkGenItemStartupCostWarm>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.startup_cost_warm = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for startup_cost_warm: {}",
                    e
                )
            });
            self
        }
        pub fn startup_time_hot<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.startup_time_hot = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for startup_time_hot: {}",
                    e
                )
            });
            self
        }
        pub fn startup_time_warm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.startup_time_warm = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for startup_time_warm: {}",
                    e
                )
            });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Status>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
        pub fn vm_setpoint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkGenItemVmSetpoint>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.vm_setpoint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vm_setpoint: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataNetworkGenItem> for super::CtmDataNetworkGenItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataNetworkGenItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bus: value.bus?,
                cost_pg_model: value.cost_pg_model?,
                cost_pg_parameters: value.cost_pg_parameters?,
                down_time_lb: value.down_time_lb?,
                ext: value.ext?,
                forced_outage_rate: value.forced_outage_rate?,
                in_service_time_lb: value.in_service_time_lb?,
                in_service_time_ub: value.in_service_time_ub?,
                mean_time_to_failure: value.mean_time_to_failure?,
                mean_time_to_repair: value.mean_time_to_repair?,
                name: value.name?,
                nominal_mva: value.nominal_mva?,
                pg_delta_lb: value.pg_delta_lb?,
                pg_delta_ub: value.pg_delta_ub?,
                pg_lb: value.pg_lb?,
                pg_ub: value.pg_ub?,
                primary_source: value.primary_source?,
                primary_source_subtype: value.primary_source_subtype?,
                qg_lb: value.qg_lb?,
                qg_ub: value.qg_ub?,
                scheduled_maintenance_rate: value.scheduled_maintenance_rate?,
                service_required: value.service_required?,
                shutdown_cost: value.shutdown_cost?,
                startup_cost_cold: value.startup_cost_cold?,
                startup_cost_hot: value.startup_cost_hot?,
                startup_cost_warm: value.startup_cost_warm?,
                startup_time_hot: value.startup_time_hot?,
                startup_time_warm: value.startup_time_warm?,
                status: value.status?,
                uid: value.uid?,
                vm_setpoint: value.vm_setpoint?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataNetworkGenItem> for CtmDataNetworkGenItem {
        fn from(value: super::CtmDataNetworkGenItem) -> Self {
            Self {
                bus: Ok(value.bus),
                cost_pg_model: Ok(value.cost_pg_model),
                cost_pg_parameters: Ok(value.cost_pg_parameters),
                down_time_lb: Ok(value.down_time_lb),
                ext: Ok(value.ext),
                forced_outage_rate: Ok(value.forced_outage_rate),
                in_service_time_lb: Ok(value.in_service_time_lb),
                in_service_time_ub: Ok(value.in_service_time_ub),
                mean_time_to_failure: Ok(value.mean_time_to_failure),
                mean_time_to_repair: Ok(value.mean_time_to_repair),
                name: Ok(value.name),
                nominal_mva: Ok(value.nominal_mva),
                pg_delta_lb: Ok(value.pg_delta_lb),
                pg_delta_ub: Ok(value.pg_delta_ub),
                pg_lb: Ok(value.pg_lb),
                pg_ub: Ok(value.pg_ub),
                primary_source: Ok(value.primary_source),
                primary_source_subtype: Ok(value.primary_source_subtype),
                qg_lb: Ok(value.qg_lb),
                qg_ub: Ok(value.qg_ub),
                scheduled_maintenance_rate: Ok(value.scheduled_maintenance_rate),
                service_required: Ok(value.service_required),
                shutdown_cost: Ok(value.shutdown_cost),
                startup_cost_cold: Ok(value.startup_cost_cold),
                startup_cost_hot: Ok(value.startup_cost_hot),
                startup_cost_warm: Ok(value.startup_cost_warm),
                startup_time_hot: Ok(value.startup_time_hot),
                startup_time_warm: Ok(value.startup_time_warm),
                status: Ok(value.status),
                uid: Ok(value.uid),
                vm_setpoint: Ok(value.vm_setpoint),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataNetworkGlobalParams {
        base_mva: ::std::result::Result<super::PositiveNumber, ::std::string::String>,
        bus_ref: ::std::result::Result<::std::option::Option<super::Uid>, ::std::string::String>,
        unit_convention: ::std::result::Result<
            super::CtmDataNetworkGlobalParamsUnitConvention,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CtmDataNetworkGlobalParams {
        fn default() -> Self {
            Self {
                base_mva: Ok(super::defaults::ctm_data_network_global_params_base_mva()),
                bus_ref: Ok(Default::default()),
                unit_convention: Err("no value supplied for unit_convention".to_string()),
            }
        }
    }
    impl CtmDataNetworkGlobalParams {
        pub fn base_mva<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PositiveNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.base_mva = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base_mva: {}", e));
            self
        }
        pub fn bus_ref<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Uid>>,
            T::Error: ::std::fmt::Display,
        {
            self.bus_ref = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bus_ref: {}", e));
            self
        }
        pub fn unit_convention<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CtmDataNetworkGlobalParamsUnitConvention>,
            T::Error: ::std::fmt::Display,
        {
            self.unit_convention = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for unit_convention: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataNetworkGlobalParams> for super::CtmDataNetworkGlobalParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataNetworkGlobalParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                base_mva: value.base_mva?,
                bus_ref: value.bus_ref?,
                unit_convention: value.unit_convention?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataNetworkGlobalParams> for CtmDataNetworkGlobalParams {
        fn from(value: super::CtmDataNetworkGlobalParams) -> Self {
            Self {
                base_mva: Ok(value.base_mva),
                bus_ref: Ok(value.bus_ref),
                unit_convention: Ok(value.unit_convention),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataNetworkHvdcP2pItem {
        base_kv_dc: ::std::result::Result<
            ::std::option::Option<super::PositiveNumber>,
            ::std::string::String,
        >,
        bus_fr: ::std::result::Result<super::Uid, ::std::string::String>,
        bus_to: ::std::result::Result<super::Uid, ::std::string::String>,
        cm_ub_fr: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkHvdcP2pItemCmUbFr>,
            ::std::string::String,
        >,
        cm_ub_to: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkHvdcP2pItemCmUbTo>,
            ::std::string::String,
        >,
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        loss_a: ::std::result::Result<super::NonnegativeNumber, ::std::string::String>,
        loss_b: ::std::result::Result<super::NonnegativeNumber, ::std::string::String>,
        loss_c: ::std::result::Result<super::NonnegativeNumber, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        nominal_mva: ::std::result::Result<
            ::std::option::Option<super::PositiveNumber>,
            ::std::string::String,
        >,
        p: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        pdc_fr_lb: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkHvdcP2pItemPdcFrLb>,
            ::std::string::String,
        >,
        pdc_fr_ub: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkHvdcP2pItemPdcFrUb>,
            ::std::string::String,
        >,
        pdc_to_lb: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkHvdcP2pItemPdcToLb>,
            ::std::string::String,
        >,
        pdc_to_ub: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkHvdcP2pItemPdcToUb>,
            ::std::string::String,
        >,
        persistent_outage_duration: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        persistent_outage_rate: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        phi_lb: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        phi_ub: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        qdc_fr_lb: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkHvdcP2pItemQdcFrLb>,
            ::std::string::String,
        >,
        qdc_fr_ub: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkHvdcP2pItemQdcFrUb>,
            ::std::string::String,
        >,
        qdc_to_lb: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkHvdcP2pItemQdcToLb>,
            ::std::string::String,
        >,
        qdc_to_ub: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkHvdcP2pItemQdcToUb>,
            ::std::string::String,
        >,
        r: ::std::result::Result<super::NonnegativeNumber, ::std::string::String>,
        sm_ub: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkHvdcP2pItemSmUb>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::Status, ::std::string::String>,
        technology: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkHvdcP2pItemTechnology>,
            ::std::string::String,
        >,
        transient_outage_rate: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
        vm_dc_lb: ::std::result::Result<
            ::std::option::Option<super::PositiveNumber>,
            ::std::string::String,
        >,
        vm_dc_ub: ::std::result::Result<
            ::std::option::Option<super::PositiveNumber>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CtmDataNetworkHvdcP2pItem {
        fn default() -> Self {
            Self {
                base_kv_dc: Ok(Default::default()),
                bus_fr: Err("no value supplied for bus_fr".to_string()),
                bus_to: Err("no value supplied for bus_to".to_string()),
                cm_ub_fr: Ok(Default::default()),
                cm_ub_to: Ok(Default::default()),
                ext: Ok(Default::default()),
                loss_a: Ok(super::defaults::ctm_data_network_hvdc_p2p_item_loss_a()),
                loss_b: Ok(super::defaults::ctm_data_network_hvdc_p2p_item_loss_b()),
                loss_c: Ok(super::defaults::ctm_data_network_hvdc_p2p_item_loss_c()),
                name: Ok(Default::default()),
                nominal_mva: Ok(Default::default()),
                p: Ok(Default::default()),
                pdc_fr_lb: Ok(Default::default()),
                pdc_fr_ub: Ok(Default::default()),
                pdc_to_lb: Ok(Default::default()),
                pdc_to_ub: Ok(Default::default()),
                persistent_outage_duration: Ok(Default::default()),
                persistent_outage_rate: Ok(Default::default()),
                phi_lb: Ok(Default::default()),
                phi_ub: Ok(Default::default()),
                qdc_fr_lb: Ok(Default::default()),
                qdc_fr_ub: Ok(Default::default()),
                qdc_to_lb: Ok(Default::default()),
                qdc_to_ub: Ok(Default::default()),
                r: Ok(super::defaults::ctm_data_network_hvdc_p2p_item_r()),
                sm_ub: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
                technology: Ok(Default::default()),
                transient_outage_rate: Ok(Default::default()),
                uid: Err("no value supplied for uid".to_string()),
                vm_dc_lb: Ok(Default::default()),
                vm_dc_ub: Ok(Default::default()),
            }
        }
    }
    impl CtmDataNetworkHvdcP2pItem {
        pub fn base_kv_dc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PositiveNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.base_kv_dc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base_kv_dc: {}", e));
            self
        }
        pub fn bus_fr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.bus_fr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bus_fr: {}", e));
            self
        }
        pub fn bus_to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.bus_to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bus_to: {}", e));
            self
        }
        pub fn cm_ub_fr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkHvdcP2pItemCmUbFr>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.cm_ub_fr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cm_ub_fr: {}", e));
            self
        }
        pub fn cm_ub_to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkHvdcP2pItemCmUbTo>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.cm_ub_to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cm_ub_to: {}", e));
            self
        }
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn loss_a<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NonnegativeNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.loss_a = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for loss_a: {}", e));
            self
        }
        pub fn loss_b<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NonnegativeNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.loss_b = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for loss_b: {}", e));
            self
        }
        pub fn loss_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NonnegativeNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.loss_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for loss_c: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn nominal_mva<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PositiveNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.nominal_mva = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nominal_mva: {}", e));
            self
        }
        pub fn p<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.p = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for p: {}", e));
            self
        }
        pub fn pdc_fr_lb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkHvdcP2pItemPdcFrLb>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.pdc_fr_lb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pdc_fr_lb: {}", e));
            self
        }
        pub fn pdc_fr_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkHvdcP2pItemPdcFrUb>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.pdc_fr_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pdc_fr_ub: {}", e));
            self
        }
        pub fn pdc_to_lb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkHvdcP2pItemPdcToLb>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.pdc_to_lb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pdc_to_lb: {}", e));
            self
        }
        pub fn pdc_to_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkHvdcP2pItemPdcToUb>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.pdc_to_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pdc_to_ub: {}", e));
            self
        }
        pub fn persistent_outage_duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.persistent_outage_duration = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for persistent_outage_duration: {}",
                    e
                )
            });
            self
        }
        pub fn persistent_outage_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.persistent_outage_rate = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for persistent_outage_rate: {}",
                    e
                )
            });
            self
        }
        pub fn phi_lb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.phi_lb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for phi_lb: {}", e));
            self
        }
        pub fn phi_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.phi_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for phi_ub: {}", e));
            self
        }
        pub fn qdc_fr_lb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkHvdcP2pItemQdcFrLb>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.qdc_fr_lb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for qdc_fr_lb: {}", e));
            self
        }
        pub fn qdc_fr_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkHvdcP2pItemQdcFrUb>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.qdc_fr_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for qdc_fr_ub: {}", e));
            self
        }
        pub fn qdc_to_lb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkHvdcP2pItemQdcToLb>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.qdc_to_lb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for qdc_to_lb: {}", e));
            self
        }
        pub fn qdc_to_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkHvdcP2pItemQdcToUb>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.qdc_to_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for qdc_to_ub: {}", e));
            self
        }
        pub fn r<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NonnegativeNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.r = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for r: {}", e));
            self
        }
        pub fn sm_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkHvdcP2pItemSmUb>>,
            T::Error: ::std::fmt::Display,
        {
            self.sm_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sm_ub: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Status>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn technology<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkHvdcP2pItemTechnology>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.technology = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for technology: {}", e));
            self
        }
        pub fn transient_outage_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.transient_outage_rate = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for transient_outage_rate: {}",
                    e
                )
            });
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
        pub fn vm_dc_lb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PositiveNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.vm_dc_lb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vm_dc_lb: {}", e));
            self
        }
        pub fn vm_dc_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PositiveNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.vm_dc_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vm_dc_ub: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataNetworkHvdcP2pItem> for super::CtmDataNetworkHvdcP2pItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataNetworkHvdcP2pItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                base_kv_dc: value.base_kv_dc?,
                bus_fr: value.bus_fr?,
                bus_to: value.bus_to?,
                cm_ub_fr: value.cm_ub_fr?,
                cm_ub_to: value.cm_ub_to?,
                ext: value.ext?,
                loss_a: value.loss_a?,
                loss_b: value.loss_b?,
                loss_c: value.loss_c?,
                name: value.name?,
                nominal_mva: value.nominal_mva?,
                p: value.p?,
                pdc_fr_lb: value.pdc_fr_lb?,
                pdc_fr_ub: value.pdc_fr_ub?,
                pdc_to_lb: value.pdc_to_lb?,
                pdc_to_ub: value.pdc_to_ub?,
                persistent_outage_duration: value.persistent_outage_duration?,
                persistent_outage_rate: value.persistent_outage_rate?,
                phi_lb: value.phi_lb?,
                phi_ub: value.phi_ub?,
                qdc_fr_lb: value.qdc_fr_lb?,
                qdc_fr_ub: value.qdc_fr_ub?,
                qdc_to_lb: value.qdc_to_lb?,
                qdc_to_ub: value.qdc_to_ub?,
                r: value.r?,
                sm_ub: value.sm_ub?,
                status: value.status?,
                technology: value.technology?,
                transient_outage_rate: value.transient_outage_rate?,
                uid: value.uid?,
                vm_dc_lb: value.vm_dc_lb?,
                vm_dc_ub: value.vm_dc_ub?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataNetworkHvdcP2pItem> for CtmDataNetworkHvdcP2pItem {
        fn from(value: super::CtmDataNetworkHvdcP2pItem) -> Self {
            Self {
                base_kv_dc: Ok(value.base_kv_dc),
                bus_fr: Ok(value.bus_fr),
                bus_to: Ok(value.bus_to),
                cm_ub_fr: Ok(value.cm_ub_fr),
                cm_ub_to: Ok(value.cm_ub_to),
                ext: Ok(value.ext),
                loss_a: Ok(value.loss_a),
                loss_b: Ok(value.loss_b),
                loss_c: Ok(value.loss_c),
                name: Ok(value.name),
                nominal_mva: Ok(value.nominal_mva),
                p: Ok(value.p),
                pdc_fr_lb: Ok(value.pdc_fr_lb),
                pdc_fr_ub: Ok(value.pdc_fr_ub),
                pdc_to_lb: Ok(value.pdc_to_lb),
                pdc_to_ub: Ok(value.pdc_to_ub),
                persistent_outage_duration: Ok(value.persistent_outage_duration),
                persistent_outage_rate: Ok(value.persistent_outage_rate),
                phi_lb: Ok(value.phi_lb),
                phi_ub: Ok(value.phi_ub),
                qdc_fr_lb: Ok(value.qdc_fr_lb),
                qdc_fr_ub: Ok(value.qdc_fr_ub),
                qdc_to_lb: Ok(value.qdc_to_lb),
                qdc_to_ub: Ok(value.qdc_to_ub),
                r: Ok(value.r),
                sm_ub: Ok(value.sm_ub),
                status: Ok(value.status),
                technology: Ok(value.technology),
                transient_outage_rate: Ok(value.transient_outage_rate),
                uid: Ok(value.uid),
                vm_dc_lb: Ok(value.vm_dc_lb),
                vm_dc_ub: Ok(value.vm_dc_ub),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataNetworkLoadItem {
        bus: ::std::result::Result<super::Uid, ::std::string::String>,
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        nominal_mva: ::std::result::Result<
            ::std::option::Option<super::PositiveNumber>,
            ::std::string::String,
        >,
        pd: ::std::result::Result<super::CtmDataNetworkLoadItemPd, ::std::string::String>,
        pd_i: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkLoadItemPdI>,
            ::std::string::String,
        >,
        pd_y: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkLoadItemPdY>,
            ::std::string::String,
        >,
        qd: ::std::result::Result<super::CtmDataNetworkLoadItemQd, ::std::string::String>,
        qd_i: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkLoadItemQdI>,
            ::std::string::String,
        >,
        qd_y: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkLoadItemQdY>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::Status, ::std::string::String>,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
    }
    impl ::std::default::Default for CtmDataNetworkLoadItem {
        fn default() -> Self {
            Self {
                bus: Err("no value supplied for bus".to_string()),
                ext: Ok(Default::default()),
                name: Ok(Default::default()),
                nominal_mva: Ok(Default::default()),
                pd: Err("no value supplied for pd".to_string()),
                pd_i: Ok(Default::default()),
                pd_y: Ok(Default::default()),
                qd: Err("no value supplied for qd".to_string()),
                qd_i: Ok(Default::default()),
                qd_y: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
                uid: Err("no value supplied for uid".to_string()),
            }
        }
    }
    impl CtmDataNetworkLoadItem {
        pub fn bus<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.bus = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bus: {}", e));
            self
        }
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn nominal_mva<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PositiveNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.nominal_mva = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nominal_mva: {}", e));
            self
        }
        pub fn pd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CtmDataNetworkLoadItemPd>,
            T::Error: ::std::fmt::Display,
        {
            self.pd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pd: {}", e));
            self
        }
        pub fn pd_i<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkLoadItemPdI>>,
            T::Error: ::std::fmt::Display,
        {
            self.pd_i = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pd_i: {}", e));
            self
        }
        pub fn pd_y<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkLoadItemPdY>>,
            T::Error: ::std::fmt::Display,
        {
            self.pd_y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pd_y: {}", e));
            self
        }
        pub fn qd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CtmDataNetworkLoadItemQd>,
            T::Error: ::std::fmt::Display,
        {
            self.qd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for qd: {}", e));
            self
        }
        pub fn qd_i<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkLoadItemQdI>>,
            T::Error: ::std::fmt::Display,
        {
            self.qd_i = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for qd_i: {}", e));
            self
        }
        pub fn qd_y<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkLoadItemQdY>>,
            T::Error: ::std::fmt::Display,
        {
            self.qd_y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for qd_y: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Status>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataNetworkLoadItem> for super::CtmDataNetworkLoadItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataNetworkLoadItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bus: value.bus?,
                ext: value.ext?,
                name: value.name?,
                nominal_mva: value.nominal_mva?,
                pd: value.pd?,
                pd_i: value.pd_i?,
                pd_y: value.pd_y?,
                qd: value.qd?,
                qd_i: value.qd_i?,
                qd_y: value.qd_y?,
                status: value.status?,
                uid: value.uid?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataNetworkLoadItem> for CtmDataNetworkLoadItem {
        fn from(value: super::CtmDataNetworkLoadItem) -> Self {
            Self {
                bus: Ok(value.bus),
                ext: Ok(value.ext),
                name: Ok(value.name),
                nominal_mva: Ok(value.nominal_mva),
                pd: Ok(value.pd),
                pd_i: Ok(value.pd_i),
                pd_y: Ok(value.pd_y),
                qd: Ok(value.qd),
                qd_i: Ok(value.qd_i),
                qd_y: Ok(value.qd_y),
                status: Ok(value.status),
                uid: Ok(value.uid),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataNetworkReserveItem {
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        participants: ::std::result::Result<::std::vec::Vec<super::Uid>, ::std::string::String>,
        pg_down: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkReserveItemPgDown>,
            ::std::string::String,
        >,
        pg_up: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkReserveItemPgUp>,
            ::std::string::String,
        >,
        reserve_type: ::std::result::Result<
            super::CtmDataNetworkReserveItemReserveType,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::Status, ::std::string::String>,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
    }
    impl ::std::default::Default for CtmDataNetworkReserveItem {
        fn default() -> Self {
            Self {
                ext: Ok(Default::default()),
                name: Ok(Default::default()),
                participants: Ok(Default::default()),
                pg_down: Ok(Default::default()),
                pg_up: Ok(Default::default()),
                reserve_type: Err("no value supplied for reserve_type".to_string()),
                status: Err("no value supplied for status".to_string()),
                uid: Err("no value supplied for uid".to_string()),
            }
        }
    }
    impl CtmDataNetworkReserveItem {
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn participants<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Uid>>,
            T::Error: ::std::fmt::Display,
        {
            self.participants = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for participants: {}", e));
            self
        }
        pub fn pg_down<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkReserveItemPgDown>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.pg_down = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pg_down: {}", e));
            self
        }
        pub fn pg_up<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkReserveItemPgUp>>,
            T::Error: ::std::fmt::Display,
        {
            self.pg_up = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pg_up: {}", e));
            self
        }
        pub fn reserve_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CtmDataNetworkReserveItemReserveType>,
            T::Error: ::std::fmt::Display,
        {
            self.reserve_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reserve_type: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Status>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataNetworkReserveItem> for super::CtmDataNetworkReserveItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataNetworkReserveItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ext: value.ext?,
                name: value.name?,
                participants: value.participants?,
                pg_down: value.pg_down?,
                pg_up: value.pg_up?,
                reserve_type: value.reserve_type?,
                status: value.status?,
                uid: value.uid?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataNetworkReserveItem> for CtmDataNetworkReserveItem {
        fn from(value: super::CtmDataNetworkReserveItem) -> Self {
            Self {
                ext: Ok(value.ext),
                name: Ok(value.name),
                participants: Ok(value.participants),
                pg_down: Ok(value.pg_down),
                pg_up: Ok(value.pg_up),
                reserve_type: Ok(value.reserve_type),
                status: Ok(value.status),
                uid: Ok(value.uid),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataNetworkShuntItem {
        bs: ::std::result::Result<super::CtmDataNetworkShuntItemBs, ::std::string::String>,
        bus: ::std::result::Result<super::Uid, ::std::string::String>,
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        gs: ::std::result::Result<super::CtmDataNetworkShuntItemGs, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        nominal_mva: ::std::result::Result<
            ::std::option::Option<super::PositiveNumber>,
            ::std::string::String,
        >,
        num_steps_ub:
            ::std::result::Result<super::CtmDataNetworkShuntItemNumStepsUb, ::std::string::String>,
        status: ::std::result::Result<super::Status, ::std::string::String>,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
    }
    impl ::std::default::Default for CtmDataNetworkShuntItem {
        fn default() -> Self {
            Self {
                bs: Err("no value supplied for bs".to_string()),
                bus: Err("no value supplied for bus".to_string()),
                ext: Ok(Default::default()),
                gs: Err("no value supplied for gs".to_string()),
                name: Ok(Default::default()),
                nominal_mva: Ok(Default::default()),
                num_steps_ub: Err("no value supplied for num_steps_ub".to_string()),
                status: Err("no value supplied for status".to_string()),
                uid: Err("no value supplied for uid".to_string()),
            }
        }
    }
    impl CtmDataNetworkShuntItem {
        pub fn bs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CtmDataNetworkShuntItemBs>,
            T::Error: ::std::fmt::Display,
        {
            self.bs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bs: {}", e));
            self
        }
        pub fn bus<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.bus = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bus: {}", e));
            self
        }
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn gs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CtmDataNetworkShuntItemGs>,
            T::Error: ::std::fmt::Display,
        {
            self.gs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gs: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn nominal_mva<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PositiveNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.nominal_mva = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nominal_mva: {}", e));
            self
        }
        pub fn num_steps_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CtmDataNetworkShuntItemNumStepsUb>,
            T::Error: ::std::fmt::Display,
        {
            self.num_steps_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for num_steps_ub: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Status>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataNetworkShuntItem> for super::CtmDataNetworkShuntItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataNetworkShuntItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bs: value.bs?,
                bus: value.bus?,
                ext: value.ext?,
                gs: value.gs?,
                name: value.name?,
                nominal_mva: value.nominal_mva?,
                num_steps_ub: value.num_steps_ub?,
                status: value.status?,
                uid: value.uid?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataNetworkShuntItem> for CtmDataNetworkShuntItem {
        fn from(value: super::CtmDataNetworkShuntItem) -> Self {
            Self {
                bs: Ok(value.bs),
                bus: Ok(value.bus),
                ext: Ok(value.ext),
                gs: Ok(value.gs),
                name: Ok(value.name),
                nominal_mva: Ok(value.nominal_mva),
                num_steps_ub: Ok(value.num_steps_ub),
                status: Ok(value.status),
                uid: Ok(value.uid),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataNetworkStorageItem {
        bus: ::std::result::Result<super::Uid, ::std::string::String>,
        charge_efficiency: ::std::result::Result<
            super::CtmDataNetworkStorageItemChargeEfficiency,
            ::std::string::String,
        >,
        charge_ub: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkStorageItemChargeUb>,
            ::std::string::String,
        >,
        cm_ub: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        discharge_efficiency: ::std::result::Result<
            super::CtmDataNetworkStorageItemDischargeEfficiency,
            ::std::string::String,
        >,
        discharge_ub: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkStorageItemDischargeUb>,
            ::std::string::String,
        >,
        energy_ub: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        nominal_mva: ::std::result::Result<
            ::std::option::Option<super::PositiveNumber>,
            ::std::string::String,
        >,
        ps_delta_lb: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        ps_delta_ub: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        ps_ex: ::std::result::Result<f64, ::std::string::String>,
        qs_ex: ::std::result::Result<f64, ::std::string::String>,
        qs_lb: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkStorageItemQsLb>,
            ::std::string::String,
        >,
        qs_ub: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkStorageItemQsUb>,
            ::std::string::String,
        >,
        sm_ub: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::Status, ::std::string::String>,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
    }
    impl ::std::default::Default for CtmDataNetworkStorageItem {
        fn default() -> Self {
            Self {
                bus: Err("no value supplied for bus".to_string()),
                charge_efficiency: Err("no value supplied for charge_efficiency".to_string()),
                charge_ub: Ok(Default::default()),
                cm_ub: Ok(Default::default()),
                discharge_efficiency: Err("no value supplied for discharge_efficiency".to_string()),
                discharge_ub: Ok(Default::default()),
                energy_ub: Ok(Default::default()),
                ext: Ok(Default::default()),
                name: Ok(Default::default()),
                nominal_mva: Ok(Default::default()),
                ps_delta_lb: Ok(Default::default()),
                ps_delta_ub: Ok(Default::default()),
                ps_ex: Err("no value supplied for ps_ex".to_string()),
                qs_ex: Err("no value supplied for qs_ex".to_string()),
                qs_lb: Ok(Default::default()),
                qs_ub: Ok(Default::default()),
                sm_ub: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
                uid: Err("no value supplied for uid".to_string()),
            }
        }
    }
    impl CtmDataNetworkStorageItem {
        pub fn bus<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.bus = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bus: {}", e));
            self
        }
        pub fn charge_efficiency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CtmDataNetworkStorageItemChargeEfficiency>,
            T::Error: ::std::fmt::Display,
        {
            self.charge_efficiency = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for charge_efficiency: {}",
                    e
                )
            });
            self
        }
        pub fn charge_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkStorageItemChargeUb>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.charge_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for charge_ub: {}", e));
            self
        }
        pub fn cm_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.cm_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cm_ub: {}", e));
            self
        }
        pub fn discharge_efficiency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CtmDataNetworkStorageItemDischargeEfficiency>,
            T::Error: ::std::fmt::Display,
        {
            self.discharge_efficiency = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for discharge_efficiency: {}",
                    e
                )
            });
            self
        }
        pub fn discharge_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkStorageItemDischargeUb>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.discharge_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for discharge_ub: {}", e));
            self
        }
        pub fn energy_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.energy_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for energy_ub: {}", e));
            self
        }
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn nominal_mva<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PositiveNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.nominal_mva = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nominal_mva: {}", e));
            self
        }
        pub fn ps_delta_lb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.ps_delta_lb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ps_delta_lb: {}", e));
            self
        }
        pub fn ps_delta_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.ps_delta_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ps_delta_ub: {}", e));
            self
        }
        pub fn ps_ex<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.ps_ex = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ps_ex: {}", e));
            self
        }
        pub fn qs_ex<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.qs_ex = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for qs_ex: {}", e));
            self
        }
        pub fn qs_lb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkStorageItemQsLb>>,
            T::Error: ::std::fmt::Display,
        {
            self.qs_lb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for qs_lb: {}", e));
            self
        }
        pub fn qs_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CtmDataNetworkStorageItemQsUb>>,
            T::Error: ::std::fmt::Display,
        {
            self.qs_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for qs_ub: {}", e));
            self
        }
        pub fn sm_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.sm_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sm_ub: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Status>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataNetworkStorageItem> for super::CtmDataNetworkStorageItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataNetworkStorageItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bus: value.bus?,
                charge_efficiency: value.charge_efficiency?,
                charge_ub: value.charge_ub?,
                cm_ub: value.cm_ub?,
                discharge_efficiency: value.discharge_efficiency?,
                discharge_ub: value.discharge_ub?,
                energy_ub: value.energy_ub?,
                ext: value.ext?,
                name: value.name?,
                nominal_mva: value.nominal_mva?,
                ps_delta_lb: value.ps_delta_lb?,
                ps_delta_ub: value.ps_delta_ub?,
                ps_ex: value.ps_ex?,
                qs_ex: value.qs_ex?,
                qs_lb: value.qs_lb?,
                qs_ub: value.qs_ub?,
                sm_ub: value.sm_ub?,
                status: value.status?,
                uid: value.uid?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataNetworkStorageItem> for CtmDataNetworkStorageItem {
        fn from(value: super::CtmDataNetworkStorageItem) -> Self {
            Self {
                bus: Ok(value.bus),
                charge_efficiency: Ok(value.charge_efficiency),
                charge_ub: Ok(value.charge_ub),
                cm_ub: Ok(value.cm_ub),
                discharge_efficiency: Ok(value.discharge_efficiency),
                discharge_ub: Ok(value.discharge_ub),
                energy_ub: Ok(value.energy_ub),
                ext: Ok(value.ext),
                name: Ok(value.name),
                nominal_mva: Ok(value.nominal_mva),
                ps_delta_lb: Ok(value.ps_delta_lb),
                ps_delta_ub: Ok(value.ps_delta_ub),
                ps_ex: Ok(value.ps_ex),
                qs_ex: Ok(value.qs_ex),
                qs_lb: Ok(value.qs_lb),
                qs_ub: Ok(value.qs_ub),
                sm_ub: Ok(value.sm_ub),
                status: Ok(value.status),
                uid: Ok(value.uid),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataNetworkSwitchItem {
        bus_fr: ::std::result::Result<super::Uid, ::std::string::String>,
        bus_to: ::std::result::Result<super::Uid, ::std::string::String>,
        cm_ub: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        nominal_mva: ::std::result::Result<
            ::std::option::Option<super::PositiveNumber>,
            ::std::string::String,
        >,
        sm_ub: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::Status, ::std::string::String>,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
    }
    impl ::std::default::Default for CtmDataNetworkSwitchItem {
        fn default() -> Self {
            Self {
                bus_fr: Err("no value supplied for bus_fr".to_string()),
                bus_to: Err("no value supplied for bus_to".to_string()),
                cm_ub: Ok(Default::default()),
                ext: Ok(Default::default()),
                name: Ok(Default::default()),
                nominal_mva: Ok(Default::default()),
                sm_ub: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
                uid: Err("no value supplied for uid".to_string()),
            }
        }
    }
    impl CtmDataNetworkSwitchItem {
        pub fn bus_fr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.bus_fr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bus_fr: {}", e));
            self
        }
        pub fn bus_to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.bus_to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bus_to: {}", e));
            self
        }
        pub fn cm_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.cm_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cm_ub: {}", e));
            self
        }
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn nominal_mva<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PositiveNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.nominal_mva = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nominal_mva: {}", e));
            self
        }
        pub fn sm_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.sm_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sm_ub: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Status>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataNetworkSwitchItem> for super::CtmDataNetworkSwitchItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataNetworkSwitchItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bus_fr: value.bus_fr?,
                bus_to: value.bus_to?,
                cm_ub: value.cm_ub?,
                ext: value.ext?,
                name: value.name?,
                nominal_mva: value.nominal_mva?,
                sm_ub: value.sm_ub?,
                status: value.status?,
                uid: value.uid?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataNetworkSwitchItem> for CtmDataNetworkSwitchItem {
        fn from(value: super::CtmDataNetworkSwitchItem) -> Self {
            Self {
                bus_fr: Ok(value.bus_fr),
                bus_to: Ok(value.bus_to),
                cm_ub: Ok(value.cm_ub),
                ext: Ok(value.ext),
                name: Ok(value.name),
                nominal_mva: Ok(value.nominal_mva),
                sm_ub: Ok(value.sm_ub),
                status: Ok(value.status),
                uid: Ok(value.uid),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataNetworkTransformerItem {
        b: ::std::result::Result<f64, ::std::string::String>,
        bus_fr: ::std::result::Result<super::Uid, ::std::string::String>,
        bus_to: ::std::result::Result<super::Uid, ::std::string::String>,
        cm_ub_a: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkTransformerItemCmUbA>,
            ::std::string::String,
        >,
        cm_ub_b: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkTransformerItemCmUbB>,
            ::std::string::String,
        >,
        cm_ub_c: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkTransformerItemCmUbC>,
            ::std::string::String,
        >,
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        g: ::std::result::Result<f64, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        nominal_mva: ::std::result::Result<
            ::std::option::Option<super::PositiveNumber>,
            ::std::string::String,
        >,
        persistent_outage_duration: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        persistent_outage_rate: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        r: ::std::result::Result<f64, ::std::string::String>,
        sm_ub_a: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkTransformerItemSmUbA>,
            ::std::string::String,
        >,
        sm_ub_b: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkTransformerItemSmUbB>,
            ::std::string::String,
        >,
        sm_ub_c: ::std::result::Result<
            ::std::option::Option<super::CtmDataNetworkTransformerItemSmUbC>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::Status, ::std::string::String>,
        ta_lb: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        ta_steps: ::std::result::Result<super::PositiveInteger, ::std::string::String>,
        ta_ub: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tm_lb: ::std::result::Result<super::PositiveNumber, ::std::string::String>,
        tm_steps: ::std::result::Result<super::PositiveInteger, ::std::string::String>,
        tm_ub: ::std::result::Result<super::PositiveNumber, ::std::string::String>,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
        x: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for CtmDataNetworkTransformerItem {
        fn default() -> Self {
            Self {
                b: Err("no value supplied for b".to_string()),
                bus_fr: Err("no value supplied for bus_fr".to_string()),
                bus_to: Err("no value supplied for bus_to".to_string()),
                cm_ub_a: Ok(Default::default()),
                cm_ub_b: Ok(Default::default()),
                cm_ub_c: Ok(Default::default()),
                ext: Ok(Default::default()),
                g: Err("no value supplied for g".to_string()),
                name: Ok(Default::default()),
                nominal_mva: Ok(Default::default()),
                persistent_outage_duration: Ok(Default::default()),
                persistent_outage_rate: Ok(Default::default()),
                r: Err("no value supplied for r".to_string()),
                sm_ub_a: Ok(Default::default()),
                sm_ub_b: Ok(Default::default()),
                sm_ub_c: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
                ta_lb: Ok(Default::default()),
                ta_steps: Ok(super::defaults::ctm_data_network_transformer_item_ta_steps()),
                ta_ub: Ok(Default::default()),
                tm_lb: Ok(super::defaults::ctm_data_network_transformer_item_tm_lb()),
                tm_steps: Ok(super::defaults::ctm_data_network_transformer_item_tm_steps()),
                tm_ub: Ok(super::defaults::ctm_data_network_transformer_item_tm_ub()),
                uid: Err("no value supplied for uid".to_string()),
                x: Err("no value supplied for x".to_string()),
            }
        }
    }
    impl CtmDataNetworkTransformerItem {
        pub fn b<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.b = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for b: {}", e));
            self
        }
        pub fn bus_fr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.bus_fr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bus_fr: {}", e));
            self
        }
        pub fn bus_to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.bus_to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bus_to: {}", e));
            self
        }
        pub fn cm_ub_a<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkTransformerItemCmUbA>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.cm_ub_a = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cm_ub_a: {}", e));
            self
        }
        pub fn cm_ub_b<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkTransformerItemCmUbB>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.cm_ub_b = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cm_ub_b: {}", e));
            self
        }
        pub fn cm_ub_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkTransformerItemCmUbC>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.cm_ub_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cm_ub_c: {}", e));
            self
        }
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn g<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.g = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for g: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn nominal_mva<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PositiveNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.nominal_mva = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nominal_mva: {}", e));
            self
        }
        pub fn persistent_outage_duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.persistent_outage_duration = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for persistent_outage_duration: {}",
                    e
                )
            });
            self
        }
        pub fn persistent_outage_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.persistent_outage_rate = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for persistent_outage_rate: {}",
                    e
                )
            });
            self
        }
        pub fn r<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.r = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for r: {}", e));
            self
        }
        pub fn sm_ub_a<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkTransformerItemSmUbA>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.sm_ub_a = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sm_ub_a: {}", e));
            self
        }
        pub fn sm_ub_b<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkTransformerItemSmUbB>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.sm_ub_b = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sm_ub_b: {}", e));
            self
        }
        pub fn sm_ub_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataNetworkTransformerItemSmUbC>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.sm_ub_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sm_ub_c: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Status>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn ta_lb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ta_lb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ta_lb: {}", e));
            self
        }
        pub fn ta_steps<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PositiveInteger>,
            T::Error: ::std::fmt::Display,
        {
            self.ta_steps = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ta_steps: {}", e));
            self
        }
        pub fn ta_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ta_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ta_ub: {}", e));
            self
        }
        pub fn tm_lb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PositiveNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.tm_lb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tm_lb: {}", e));
            self
        }
        pub fn tm_steps<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PositiveInteger>,
            T::Error: ::std::fmt::Display,
        {
            self.tm_steps = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tm_steps: {}", e));
            self
        }
        pub fn tm_ub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PositiveNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.tm_ub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tm_ub: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataNetworkTransformerItem>
        for super::CtmDataNetworkTransformerItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataNetworkTransformerItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                b: value.b?,
                bus_fr: value.bus_fr?,
                bus_to: value.bus_to?,
                cm_ub_a: value.cm_ub_a?,
                cm_ub_b: value.cm_ub_b?,
                cm_ub_c: value.cm_ub_c?,
                ext: value.ext?,
                g: value.g?,
                name: value.name?,
                nominal_mva: value.nominal_mva?,
                persistent_outage_duration: value.persistent_outage_duration?,
                persistent_outage_rate: value.persistent_outage_rate?,
                r: value.r?,
                sm_ub_a: value.sm_ub_a?,
                sm_ub_b: value.sm_ub_b?,
                sm_ub_c: value.sm_ub_c?,
                status: value.status?,
                ta_lb: value.ta_lb?,
                ta_steps: value.ta_steps?,
                ta_ub: value.ta_ub?,
                tm_lb: value.tm_lb?,
                tm_steps: value.tm_steps?,
                tm_ub: value.tm_ub?,
                uid: value.uid?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataNetworkTransformerItem> for CtmDataNetworkTransformerItem {
        fn from(value: super::CtmDataNetworkTransformerItem) -> Self {
            Self {
                b: Ok(value.b),
                bus_fr: Ok(value.bus_fr),
                bus_to: Ok(value.bus_to),
                cm_ub_a: Ok(value.cm_ub_a),
                cm_ub_b: Ok(value.cm_ub_b),
                cm_ub_c: Ok(value.cm_ub_c),
                ext: Ok(value.ext),
                g: Ok(value.g),
                name: Ok(value.name),
                nominal_mva: Ok(value.nominal_mva),
                persistent_outage_duration: Ok(value.persistent_outage_duration),
                persistent_outage_rate: Ok(value.persistent_outage_rate),
                r: Ok(value.r),
                sm_ub_a: Ok(value.sm_ub_a),
                sm_ub_b: Ok(value.sm_ub_b),
                sm_ub_c: Ok(value.sm_ub_c),
                status: Ok(value.status),
                ta_lb: Ok(value.ta_lb),
                ta_steps: Ok(value.ta_steps),
                ta_ub: Ok(value.ta_ub),
                tm_lb: Ok(value.tm_lb),
                tm_steps: Ok(value.tm_steps),
                tm_ub: Ok(value.tm_ub),
                uid: Ok(value.uid),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataNetworkZoneItem {
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::Status, ::std::string::String>,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
    }
    impl ::std::default::Default for CtmDataNetworkZoneItem {
        fn default() -> Self {
            Self {
                ext: Ok(Default::default()),
                name: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
                uid: Err("no value supplied for uid".to_string()),
            }
        }
    }
    impl CtmDataNetworkZoneItem {
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Status>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataNetworkZoneItem> for super::CtmDataNetworkZoneItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataNetworkZoneItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ext: value.ext?,
                name: value.name?,
                status: value.status?,
                uid: value.uid?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataNetworkZoneItem> for CtmDataNetworkZoneItem {
        fn from(value: super::CtmDataNetworkZoneItem) -> Self {
            Self {
                ext: Ok(value.ext),
                name: Ok(value.name),
                status: Ok(value.status),
                uid: Ok(value.uid),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataTemporalBoundary {
        bus: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataTemporalBoundaryBusItem>,
            ::std::string::String,
        >,
        gen: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataTemporalBoundaryGenItem>,
            ::std::string::String,
        >,
        global_params: ::std::result::Result<
            super::CtmDataTemporalBoundaryGlobalParams,
            ::std::string::String,
        >,
        hvdc_p2p: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataTemporalBoundaryHvdcP2pItem>,
            ::std::string::String,
        >,
        shunt: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataTemporalBoundaryShuntItem>,
            ::std::string::String,
        >,
        storage: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataTemporalBoundaryStorageItem>,
            ::std::string::String,
        >,
        switch: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataTemporalBoundarySwitchItem>,
            ::std::string::String,
        >,
        transformer: ::std::result::Result<
            ::std::vec::Vec<super::CtmDataTemporalBoundaryTransformerItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CtmDataTemporalBoundary {
        fn default() -> Self {
            Self {
                bus: Ok(Default::default()),
                gen: Ok(Default::default()),
                global_params: Err("no value supplied for global_params".to_string()),
                hvdc_p2p: Ok(Default::default()),
                shunt: Ok(Default::default()),
                storage: Ok(Default::default()),
                switch: Ok(Default::default()),
                transformer: Ok(Default::default()),
            }
        }
    }
    impl CtmDataTemporalBoundary {
        pub fn bus<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CtmDataTemporalBoundaryBusItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.bus = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bus: {}", e));
            self
        }
        pub fn gen<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CtmDataTemporalBoundaryGenItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.gen = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gen: {}", e));
            self
        }
        pub fn global_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CtmDataTemporalBoundaryGlobalParams>,
            T::Error: ::std::fmt::Display,
        {
            self.global_params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for global_params: {}", e));
            self
        }
        pub fn hvdc_p2p<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CtmDataTemporalBoundaryHvdcP2pItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.hvdc_p2p = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hvdc_p2p: {}", e));
            self
        }
        pub fn shunt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CtmDataTemporalBoundaryShuntItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.shunt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shunt: {}", e));
            self
        }
        pub fn storage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CtmDataTemporalBoundaryStorageItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.storage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for storage: {}", e));
            self
        }
        pub fn switch<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CtmDataTemporalBoundarySwitchItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.switch = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for switch: {}", e));
            self
        }
        pub fn transformer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::CtmDataTemporalBoundaryTransformerItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.transformer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for transformer: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataTemporalBoundary> for super::CtmDataTemporalBoundary {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataTemporalBoundary,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bus: value.bus?,
                gen: value.gen?,
                global_params: value.global_params?,
                hvdc_p2p: value.hvdc_p2p?,
                shunt: value.shunt?,
                storage: value.storage?,
                switch: value.switch?,
                transformer: value.transformer?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataTemporalBoundary> for CtmDataTemporalBoundary {
        fn from(value: super::CtmDataTemporalBoundary) -> Self {
            Self {
                bus: Ok(value.bus),
                gen: Ok(value.gen),
                global_params: Ok(value.global_params),
                hvdc_p2p: Ok(value.hvdc_p2p),
                shunt: Ok(value.shunt),
                storage: Ok(value.storage),
                switch: Ok(value.switch),
                transformer: Ok(value.transformer),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataTemporalBoundaryBusItem {
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
        va: ::std::result::Result<f64, ::std::string::String>,
        vm: ::std::result::Result<
            ::std::option::Option<super::PositiveNumber>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CtmDataTemporalBoundaryBusItem {
        fn default() -> Self {
            Self {
                ext: Ok(Default::default()),
                uid: Err("no value supplied for uid".to_string()),
                va: Err("no value supplied for va".to_string()),
                vm: Ok(Default::default()),
            }
        }
    }
    impl CtmDataTemporalBoundaryBusItem {
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
        pub fn va<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.va = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for va: {}", e));
            self
        }
        pub fn vm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PositiveNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.vm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vm: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataTemporalBoundaryBusItem>
        for super::CtmDataTemporalBoundaryBusItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataTemporalBoundaryBusItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ext: value.ext?,
                uid: value.uid?,
                va: value.va?,
                vm: value.vm?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataTemporalBoundaryBusItem>
        for CtmDataTemporalBoundaryBusItem
    {
        fn from(value: super::CtmDataTemporalBoundaryBusItem) -> Self {
            Self {
                ext: Ok(value.ext),
                uid: Ok(value.uid),
                va: Ok(value.va),
                vm: Ok(value.vm),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataTemporalBoundaryGenItem {
        down_time: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        in_service_time: ::std::result::Result<
            ::std::option::Option<super::NonnegativeNumber>,
            ::std::string::String,
        >,
        pg: ::std::result::Result<f64, ::std::string::String>,
        qg: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
    }
    impl ::std::default::Default for CtmDataTemporalBoundaryGenItem {
        fn default() -> Self {
            Self {
                down_time: Ok(Default::default()),
                ext: Ok(Default::default()),
                in_service_time: Ok(Default::default()),
                pg: Err("no value supplied for pg".to_string()),
                qg: Ok(Default::default()),
                uid: Err("no value supplied for uid".to_string()),
            }
        }
    }
    impl CtmDataTemporalBoundaryGenItem {
        pub fn down_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.down_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for down_time: {}", e));
            self
        }
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn in_service_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.in_service_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for in_service_time: {}", e));
            self
        }
        pub fn pg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.pg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pg: {}", e));
            self
        }
        pub fn qg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.qg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for qg: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataTemporalBoundaryGenItem>
        for super::CtmDataTemporalBoundaryGenItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataTemporalBoundaryGenItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                down_time: value.down_time?,
                ext: value.ext?,
                in_service_time: value.in_service_time?,
                pg: value.pg?,
                qg: value.qg?,
                uid: value.uid?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataTemporalBoundaryGenItem>
        for CtmDataTemporalBoundaryGenItem
    {
        fn from(value: super::CtmDataTemporalBoundaryGenItem) -> Self {
            Self {
                down_time: Ok(value.down_time),
                ext: Ok(value.ext),
                in_service_time: Ok(value.in_service_time),
                pg: Ok(value.pg),
                qg: Ok(value.qg),
                uid: Ok(value.uid),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataTemporalBoundaryGlobalParams {
        time_elapsed: ::std::result::Result<super::NonnegativeNumber, ::std::string::String>,
    }
    impl ::std::default::Default for CtmDataTemporalBoundaryGlobalParams {
        fn default() -> Self {
            Self {
                time_elapsed: Err("no value supplied for time_elapsed".to_string()),
            }
        }
    }
    impl CtmDataTemporalBoundaryGlobalParams {
        pub fn time_elapsed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NonnegativeNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.time_elapsed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_elapsed: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataTemporalBoundaryGlobalParams>
        for super::CtmDataTemporalBoundaryGlobalParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataTemporalBoundaryGlobalParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                time_elapsed: value.time_elapsed?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataTemporalBoundaryGlobalParams>
        for CtmDataTemporalBoundaryGlobalParams
    {
        fn from(value: super::CtmDataTemporalBoundaryGlobalParams) -> Self {
            Self {
                time_elapsed: Ok(value.time_elapsed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataTemporalBoundaryHvdcP2pItem {
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        pdc_fr: ::std::result::Result<f64, ::std::string::String>,
        pdc_to: ::std::result::Result<f64, ::std::string::String>,
        qdc_fr: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        qdc_to: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
        vm_dc_fr: ::std::result::Result<
            ::std::option::Option<super::PositiveNumber>,
            ::std::string::String,
        >,
        vm_dc_to: ::std::result::Result<
            ::std::option::Option<super::PositiveNumber>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CtmDataTemporalBoundaryHvdcP2pItem {
        fn default() -> Self {
            Self {
                ext: Ok(Default::default()),
                pdc_fr: Err("no value supplied for pdc_fr".to_string()),
                pdc_to: Err("no value supplied for pdc_to".to_string()),
                qdc_fr: Ok(Default::default()),
                qdc_to: Ok(Default::default()),
                uid: Err("no value supplied for uid".to_string()),
                vm_dc_fr: Ok(Default::default()),
                vm_dc_to: Ok(Default::default()),
            }
        }
    }
    impl CtmDataTemporalBoundaryHvdcP2pItem {
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn pdc_fr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.pdc_fr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pdc_fr: {}", e));
            self
        }
        pub fn pdc_to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.pdc_to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pdc_to: {}", e));
            self
        }
        pub fn qdc_fr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.qdc_fr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for qdc_fr: {}", e));
            self
        }
        pub fn qdc_to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.qdc_to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for qdc_to: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
        pub fn vm_dc_fr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PositiveNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.vm_dc_fr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vm_dc_fr: {}", e));
            self
        }
        pub fn vm_dc_to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PositiveNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.vm_dc_to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vm_dc_to: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataTemporalBoundaryHvdcP2pItem>
        for super::CtmDataTemporalBoundaryHvdcP2pItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataTemporalBoundaryHvdcP2pItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ext: value.ext?,
                pdc_fr: value.pdc_fr?,
                pdc_to: value.pdc_to?,
                qdc_fr: value.qdc_fr?,
                qdc_to: value.qdc_to?,
                uid: value.uid?,
                vm_dc_fr: value.vm_dc_fr?,
                vm_dc_to: value.vm_dc_to?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataTemporalBoundaryHvdcP2pItem>
        for CtmDataTemporalBoundaryHvdcP2pItem
    {
        fn from(value: super::CtmDataTemporalBoundaryHvdcP2pItem) -> Self {
            Self {
                ext: Ok(value.ext),
                pdc_fr: Ok(value.pdc_fr),
                pdc_to: Ok(value.pdc_to),
                qdc_fr: Ok(value.qdc_fr),
                qdc_to: Ok(value.qdc_to),
                uid: Ok(value.uid),
                vm_dc_fr: Ok(value.vm_dc_fr),
                vm_dc_to: Ok(value.vm_dc_to),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataTemporalBoundaryShuntItem {
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        num_steps: ::std::result::Result<
            super::CtmDataTemporalBoundaryShuntItemNumSteps,
            ::std::string::String,
        >,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
    }
    impl ::std::default::Default for CtmDataTemporalBoundaryShuntItem {
        fn default() -> Self {
            Self {
                ext: Ok(Default::default()),
                num_steps: Err("no value supplied for num_steps".to_string()),
                uid: Err("no value supplied for uid".to_string()),
            }
        }
    }
    impl CtmDataTemporalBoundaryShuntItem {
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn num_steps<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CtmDataTemporalBoundaryShuntItemNumSteps>,
            T::Error: ::std::fmt::Display,
        {
            self.num_steps = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for num_steps: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataTemporalBoundaryShuntItem>
        for super::CtmDataTemporalBoundaryShuntItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataTemporalBoundaryShuntItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ext: value.ext?,
                num_steps: value.num_steps?,
                uid: value.uid?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataTemporalBoundaryShuntItem>
        for CtmDataTemporalBoundaryShuntItem
    {
        fn from(value: super::CtmDataTemporalBoundaryShuntItem) -> Self {
            Self {
                ext: Ok(value.ext),
                num_steps: Ok(value.num_steps),
                uid: Ok(value.uid),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataTemporalBoundaryStorageItem {
        energy: ::std::result::Result<super::NonnegativeNumber, ::std::string::String>,
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        ps: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        qs: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
    }
    impl ::std::default::Default for CtmDataTemporalBoundaryStorageItem {
        fn default() -> Self {
            Self {
                energy: Err("no value supplied for energy".to_string()),
                ext: Ok(Default::default()),
                ps: Ok(Default::default()),
                qs: Ok(Default::default()),
                uid: Err("no value supplied for uid".to_string()),
            }
        }
    }
    impl CtmDataTemporalBoundaryStorageItem {
        pub fn energy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NonnegativeNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.energy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for energy: {}", e));
            self
        }
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn ps<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ps = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ps: {}", e));
            self
        }
        pub fn qs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.qs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for qs: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataTemporalBoundaryStorageItem>
        for super::CtmDataTemporalBoundaryStorageItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataTemporalBoundaryStorageItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                energy: value.energy?,
                ext: value.ext?,
                ps: value.ps?,
                qs: value.qs?,
                uid: value.uid?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataTemporalBoundaryStorageItem>
        for CtmDataTemporalBoundaryStorageItem
    {
        fn from(value: super::CtmDataTemporalBoundaryStorageItem) -> Self {
            Self {
                energy: Ok(value.energy),
                ext: Ok(value.ext),
                ps: Ok(value.ps),
                qs: Ok(value.qs),
                uid: Ok(value.uid),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataTemporalBoundarySwitchItem {
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        state: ::std::result::Result<super::Binary, ::std::string::String>,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
    }
    impl ::std::default::Default for CtmDataTemporalBoundarySwitchItem {
        fn default() -> Self {
            Self {
                ext: Ok(Default::default()),
                state: Err("no value supplied for state".to_string()),
                uid: Err("no value supplied for uid".to_string()),
            }
        }
    }
    impl CtmDataTemporalBoundarySwitchItem {
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Binary>,
            T::Error: ::std::fmt::Display,
        {
            self.state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for state: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataTemporalBoundarySwitchItem>
        for super::CtmDataTemporalBoundarySwitchItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataTemporalBoundarySwitchItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ext: value.ext?,
                state: value.state?,
                uid: value.uid?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataTemporalBoundarySwitchItem>
        for CtmDataTemporalBoundarySwitchItem
    {
        fn from(value: super::CtmDataTemporalBoundarySwitchItem) -> Self {
            Self {
                ext: Ok(value.ext),
                state: Ok(value.state),
                uid: Ok(value.uid),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataTemporalBoundaryTransformerItem {
        ext: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        ta: ::std::result::Result<f64, ::std::string::String>,
        tm: ::std::result::Result<super::PositiveNumber, ::std::string::String>,
        uid: ::std::result::Result<super::Uid, ::std::string::String>,
    }
    impl ::std::default::Default for CtmDataTemporalBoundaryTransformerItem {
        fn default() -> Self {
            Self {
                ext: Ok(Default::default()),
                ta: Err("no value supplied for ta".to_string()),
                tm: Err("no value supplied for tm".to_string()),
                uid: Err("no value supplied for uid".to_string()),
            }
        }
    }
    impl CtmDataTemporalBoundaryTransformerItem {
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn ta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.ta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ta: {}", e));
            self
        }
        pub fn tm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PositiveNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.tm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tm: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Uid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataTemporalBoundaryTransformerItem>
        for super::CtmDataTemporalBoundaryTransformerItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataTemporalBoundaryTransformerItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ext: value.ext?,
                ta: value.ta?,
                tm: value.tm?,
                uid: value.uid?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataTemporalBoundaryTransformerItem>
        for CtmDataTemporalBoundaryTransformerItem
    {
        fn from(value: super::CtmDataTemporalBoundaryTransformerItem) -> Self {
            Self {
                ext: Ok(value.ext),
                ta: Ok(value.ta),
                tm: Ok(value.tm),
                uid: Ok(value.uid),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CtmDataTimeSeriesData {
        ext: ::std::result::Result<::std::vec::Vec<::serde_json::Value>, ::std::string::String>,
        name: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        path_to_file: ::std::result::Result<
            ::std::option::Option<super::CtmDataTimeSeriesDataPathToFile>,
            ::std::string::String,
        >,
        timestamp:
            ::std::result::Result<::std::vec::Vec<super::NonnegativeNumber>, ::std::string::String>,
        uid: ::std::result::Result<::std::vec::Vec<super::Uid>, ::std::string::String>,
        values: ::std::result::Result<
            ::std::vec::Vec<::std::vec::Vec<::serde_json::Value>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CtmDataTimeSeriesData {
        fn default() -> Self {
            Self {
                ext: Ok(Default::default()),
                name: Ok(Default::default()),
                path_to_file: Ok(Default::default()),
                timestamp: Ok(Default::default()),
                uid: Err("no value supplied for uid".to_string()),
                values: Ok(Default::default()),
            }
        }
    }
    impl CtmDataTimeSeriesData {
        pub fn ext<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.ext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn path_to_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CtmDataTimeSeriesDataPathToFile>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.path_to_file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path_to_file: {}", e));
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::NonnegativeNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timestamp: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Uid>>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
        pub fn values<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::vec::Vec<::serde_json::Value>>>,
            T::Error: ::std::fmt::Display,
        {
            self.values = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for values: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CtmDataTimeSeriesData> for super::CtmDataTimeSeriesData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CtmDataTimeSeriesData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ext: value.ext?,
                name: value.name?,
                path_to_file: value.path_to_file?,
                timestamp: value.timestamp?,
                uid: value.uid?,
                values: value.values?,
            })
        }
    }
    impl ::std::convert::From<super::CtmDataTimeSeriesData> for CtmDataTimeSeriesData {
        fn from(value: super::CtmDataTimeSeriesData) -> Self {
            Self {
                ext: Ok(value.ext),
                name: Ok(value.name),
                path_to_file: Ok(value.path_to_file),
                timestamp: Ok(value.timestamp),
                uid: Ok(value.uid),
                values: Ok(value.values),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TimeSeriesReference {
        scale_factor: ::std::result::Result<f64, ::std::string::String>,
        uid: ::std::result::Result<super::TimeSeriesReferenceUid, ::std::string::String>,
    }
    impl ::std::default::Default for TimeSeriesReference {
        fn default() -> Self {
            Self {
                scale_factor: Err("no value supplied for scale_factor".to_string()),
                uid: Err("no value supplied for uid".to_string()),
            }
        }
    }
    impl TimeSeriesReference {
        pub fn scale_factor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.scale_factor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scale_factor: {}", e));
            self
        }
        pub fn uid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TimeSeriesReferenceUid>,
            T::Error: ::std::fmt::Display,
        {
            self.uid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uid: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TimeSeriesReference> for super::TimeSeriesReference {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TimeSeriesReference,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                scale_factor: value.scale_factor?,
                uid: value.uid?,
            })
        }
    }
    impl ::std::convert::From<super::TimeSeriesReference> for TimeSeriesReference {
        fn from(value: super::TimeSeriesReference) -> Self {
            Self {
                scale_factor: Ok(value.scale_factor),
                uid: Ok(value.uid),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct XyPairs {
        x: ::std::result::Result<::std::vec::Vec<f64>, ::std::string::String>,
        y: ::std::result::Result<::std::vec::Vec<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for XyPairs {
        fn default() -> Self {
            Self {
                x: Err("no value supplied for x".to_string()),
                y: Err("no value supplied for y".to_string()),
            }
        }
    }
    impl XyPairs {
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<XyPairs> for super::XyPairs {
        type Error = super::error::ConversionError;
        fn try_from(value: XyPairs) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                x: value.x?,
                y: value.y?,
            })
        }
    }
    impl ::std::convert::From<super::XyPairs> for XyPairs {
        fn from(value: super::XyPairs) -> Self {
            Self {
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn default_i64<T, const V: i64>() -> T
    where
        T: ::std::convert::TryFrom<i64>,
        <T as ::std::convert::TryFrom<i64>>::Error: ::std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
    pub(super) fn default_nzu64<T, const V: u64>() -> T
    where
        T: ::std::convert::TryFrom<::std::num::NonZeroU64>,
        <T as ::std::convert::TryFrom<::std::num::NonZeroU64>>::Error: ::std::fmt::Debug,
    {
        T::try_from(::std::num::NonZeroU64::try_from(V).unwrap()).unwrap()
    }
    pub(super) fn ctm_data_network_global_params_base_mva() -> super::PositiveNumber {
        super::PositiveNumber(100.0_f64)
    }
    pub(super) fn ctm_data_network_hvdc_p2p_item_loss_a() -> super::NonnegativeNumber {
        super::NonnegativeNumber(0_f64)
    }
    pub(super) fn ctm_data_network_hvdc_p2p_item_loss_b() -> super::NonnegativeNumber {
        super::NonnegativeNumber(0_f64)
    }
    pub(super) fn ctm_data_network_hvdc_p2p_item_loss_c() -> super::NonnegativeNumber {
        super::NonnegativeNumber(0_f64)
    }
    pub(super) fn ctm_data_network_hvdc_p2p_item_r() -> super::NonnegativeNumber {
        super::NonnegativeNumber(0.0_f64)
    }
    pub(super) fn ctm_data_network_transformer_item_ta_steps() -> super::PositiveInteger {
        super::PositiveInteger(::std::num::NonZeroU64::new(1).unwrap())
    }
    pub(super) fn ctm_data_network_transformer_item_tm_lb() -> super::PositiveNumber {
        super::PositiveNumber(1.0_f64)
    }
    pub(super) fn ctm_data_network_transformer_item_tm_steps() -> super::PositiveInteger {
        super::PositiveInteger(::std::num::NonZeroU64::new(1).unwrap())
    }
    pub(super) fn ctm_data_network_transformer_item_tm_ub() -> super::PositiveNumber {
        super::PositiveNumber(1.0_f64)
    }
}
