use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize)]
pub enum CountryCode {
    #[doc = "Afghanistan"]
    Af,
    #[doc = "Åland Islands"]
    Ax,
    #[doc = "Albania"]
    Al,
    #[doc = "Algeria"]
    Dz,
    #[doc = "American Samoa"]
    As,
    #[doc = "Andorra"]
    Ad,
    #[doc = "Angola"]
    Ao,
    #[doc = "Anguilla"]
    Ai,
    #[doc = "Antarctica"]
    Aq,
    #[doc = "Antigua and Barbuda"]
    Ag,
    #[doc = "Argentina"]
    Ar,
    #[doc = "Armenia"]
    Am,
    #[doc = "Aruba"]
    Aw,
    #[doc = "Australia"]
    Au,
    #[doc = "Austria"]
    At,
    #[doc = "Azerbaijan"]
    Az,
    #[doc = "Bahamas"]
    Bs,
    #[doc = "Bahrain"]
    Bh,
    #[doc = "Bangladesh"]
    Bd,
    #[doc = "Barbados"]
    Bb,
    #[doc = "Belarus"]
    By,
    #[doc = "Belgium"]
    Be,
    #[doc = "Belize"]
    Bz,
    #[doc = "Benin"]
    Bj,
    #[doc = "Bermuda"]
    Bm,
    #[doc = "Bhutan"]
    Bt,
    #[doc = "Bolivia (Plurinational State of)"]
    Bo,
    #[doc = "Bonaire, Sint Eustatius and Saba"]
    Bq,
    #[doc = "Bosnia and Herzegovina"]
    Ba,
    #[doc = "Botswana"]
    Bw,
    #[doc = "Bouvet Island"]
    Bv,
    #[doc = "Brazil"]
    Br,
    #[doc = "British Indian Ocean Territory"]
    Io,
    #[doc = "Brunei Darussalam"]
    Bn,
    #[doc = "Bulgaria"]
    Bg,
    #[doc = "Burkina Faso"]
    Bf,
    #[doc = "Burundi"]
    Bi,
    #[doc = "Cabo Verde"]
    Cv,
    #[doc = "Cambodia"]
    Kh,
    #[doc = "Cameroon"]
    Cm,
    #[doc = "Canada"]
    Ca,
    #[doc = "Cayman Islands"]
    Ky,
    #[doc = "Central African Republic"]
    Cf,
    #[doc = "Chad"]
    Td,
    #[doc = "Chile"]
    Cl,
    #[doc = "China"]
    Cn,
    #[doc = "Christmas Island"]
    Cx,
    #[doc = "Cocos (Keeling) Islands"]
    Cc,
    #[doc = "Colombia"]
    Co,
    #[doc = "Comoros"]
    Km,
    #[doc = "Congo"]
    Cg,
    #[doc = "Congo (Democratic Republic of the)"]
    Cd,
    #[doc = "Cook Islands"]
    Ck,
    #[doc = "Costa Rica"]
    Cr,
    #[doc = "Côte d'Ivoire"]
    Ci,
    #[doc = "Croatia"]
    Hr,
    #[doc = "Cuba"]
    Cu,
    #[doc = "Curaçao"]
    Cw,
    #[doc = "Cyprus"]
    Cy,
    #[doc = "Czechia"]
    Cz,
    #[doc = "Denmark"]
    Dk,
    #[doc = "Djibouti"]
    Dj,
    #[doc = "Dominica"]
    Dm,
    #[doc = "Dominican Republic"]
    Do,
    #[doc = "Ecuador"]
    Ec,
    #[doc = "Egypt"]
    Eg,
    #[doc = "El Salvador"]
    Sv,
    #[doc = "Equatorial Guinea"]
    Gq,
    #[doc = "Eritrea"]
    Er,
    #[doc = "Estonia"]
    Ee,
    #[doc = "Ethiopia"]
    Et,
    #[doc = "Falkland Islands (Malvinas)"]
    Fk,
    #[doc = "Faroe Islands"]
    Fo,
    #[doc = "Fiji"]
    Fj,
    #[doc = "Finland"]
    Fi,
    #[doc = "France"]
    Fr,
    #[doc = "French Guiana"]
    Gf,
    #[doc = "French Polynesia"]
    Pf,
    #[doc = "French Southern Territories"]
    Tf,
    #[doc = "Gabon"]
    Ga,
    #[doc = "Gambia"]
    Gm,
    #[doc = "Georgia"]
    Ge,
    #[doc = "Germany"]
    De,
    #[doc = "Ghana"]
    Gh,
    #[doc = "Gibraltar"]
    Gi,
    #[doc = "Greece"]
    Gr,
    #[doc = "Greenland"]
    Gl,
    #[doc = "Grenada"]
    Gd,
    #[doc = "Guadeloupe"]
    Gp,
    #[doc = "Guam"]
    Gu,
    #[doc = "Guatemala"]
    Gt,
    #[doc = "Guernsey"]
    Gg,
    #[doc = "Guinea"]
    Gn,
    #[doc = "Guinea-Bissau"]
    Gw,
    #[doc = "Guyana"]
    Gy,
    #[doc = "Haiti"]
    Ht,
    #[doc = "Heard Island and McDonald Islands"]
    Hm,
    #[doc = "Holy See"]
    Va,
    #[doc = "Honduras"]
    Hn,
    #[doc = "Hong Kong"]
    Hk,
    #[doc = "Hungary"]
    Hu,
    #[doc = "Iceland"]
    Is,
    #[doc = "India"]
    In,
    #[doc = "Indonesia"]
    Id,
    #[doc = "Iran (Islamic Republic of)"]
    Ir,
    #[doc = "Iraq"]
    Iq,
    #[doc = "Ireland"]
    Ie,
    #[doc = "Isle of Man"]
    Im,
    #[doc = "Israel"]
    Il,
    #[doc = "Italy"]
    It,
    #[doc = "Jamaica"]
    Jm,
    #[doc = "Japan"]
    Jp,
    #[doc = "Jersey"]
    Je,
    #[doc = "Jordan"]
    Jo,
    #[doc = "Kazakhstan"]
    Kz,
    #[doc = "Kenya"]
    Ke,
    #[doc = "Kiribati"]
    Ki,
    #[doc = "Korea (Democratic People's Republic of)"]
    Kp,
    #[doc = "Korea (Republic of)"]
    Kr,
    #[doc = "Kuwait"]
    Kw,
    #[doc = "Kyrgyzstan"]
    Kg,
    #[doc = "Lao People's Democratic Republic"]
    La,
    #[doc = "Latvia"]
    Lv,
    #[doc = "Lebanon"]
    Lb,
    #[doc = "Lesotho"]
    Ls,
    #[doc = "Liberia"]
    Lr,
    #[doc = "Libya"]
    Ly,
    #[doc = "Liechtenstein"]
    Li,
    #[doc = "Lithuania"]
    Lt,
    #[doc = "Luxembourg"]
    Lu,
    #[doc = "Macao"]
    Mo,
    #[doc = "Macedonia (the former Yugoslav Republic of)"]
    Mk,
    #[doc = "Madagascar"]
    Mg,
    #[doc = "Malawi"]
    Mw,
    #[doc = "Malaysia"]
    My,
    #[doc = "Maldives"]
    Mv,
    #[doc = "Mali"]
    Ml,
    #[doc = "Malta"]
    Mt,
    #[doc = "Marshall Islands"]
    Mh,
    #[doc = "Martinique"]
    Mq,
    #[doc = "Mauritania"]
    Mr,
    #[doc = "Mauritius"]
    Mu,
    #[doc = "Mayotte"]
    Yt,
    #[doc = "Mexico"]
    Mx,
    #[doc = "Micronesia (Federated States of)"]
    Fm,
    #[doc = "Moldova (Republic of)"]
    Md,
    #[doc = "Monaco"]
    Mc,
    #[doc = "Mongolia"]
    Mn,
    #[doc = "Montenegro"]
    Me,
    #[doc = "Montserrat"]
    Ms,
    #[doc = "Morocco"]
    Ma,
    #[doc = "Mozambique"]
    Mz,
    #[doc = "Myanmar"]
    Mm,
    #[doc = "Namibia"]
    Na,
    #[doc = "Nauru"]
    Nr,
    #[doc = "Nepal"]
    Np,
    #[doc = "Netherlands"]
    Nl,
    #[doc = "New Caledonia"]
    Nc,
    #[doc = "New Zealand"]
    Nz,
    #[doc = "Nicaragua"]
    Ni,
    #[doc = "Niger"]
    Ne,
    #[doc = "Nigeria"]
    Ng,
    #[doc = "Niue"]
    Nu,
    #[doc = "Norfolk Island"]
    Nf,
    #[doc = "Northern Mariana Islands"]
    Mp,
    #[doc = "Norway"]
    No,
    #[doc = "Oman"]
    Om,
    #[doc = "Pakistan"]
    Pk,
    #[doc = "Palau"]
    Pw,
    #[doc = "Palestine, State of"]
    Ps,
    #[doc = "Panama"]
    Pa,
    #[doc = "Papua New Guinea"]
    Pg,
    #[doc = "Paraguay"]
    Py,
    #[doc = "Peru"]
    Pe,
    #[doc = "Philippines"]
    Ph,
    #[doc = "Pitcairn"]
    Pn,
    #[doc = "Poland"]
    Pl,
    #[doc = "Portugal"]
    Pt,
    #[doc = "Puerto Rico"]
    Pr,
    #[doc = "Qatar"]
    Qa,
    #[doc = "Réunion"]
    Re,
    #[doc = "Romania"]
    Ro,
    #[doc = "Russian Federation"]
    Ru,
    #[doc = "Rwanda"]
    Rw,
    #[doc = "Saint Barthélemy"]
    Bl,
    #[doc = "Saint Helena, Ascension and Tristan da Cunha"]
    Sh,
    #[doc = "Saint Kitts and Nevis"]
    Kn,
    #[doc = "Saint Lucia"]
    Lc,
    #[doc = "Saint Martin (French part)"]
    Mf,
    #[doc = "Saint Pierre and Miquelon"]
    Pm,
    #[doc = "Saint Vincent and the Grenadines"]
    Vc,
    #[doc = "Samoa"]
    Ws,
    #[doc = "San Marino"]
    Sm,
    #[doc = "Sao Tome and Principe"]
    St,
    #[doc = "Saudi Arabia"]
    Sa,
    #[doc = "Senegal"]
    Sn,
    #[doc = "Serbia"]
    Rs,
    #[doc = "Seychelles"]
    Sc,
    #[doc = "Sierra Leone"]
    Sl,
    #[doc = "Singapore"]
    Sg,
    #[doc = "Sint Maarten (Dutch part)"]
    Sx,
    #[doc = "Slovakia"]
    Sk,
    #[doc = "Slovenia"]
    Si,
    #[doc = "Solomon Islands"]
    Sb,
    #[doc = "Somalia"]
    So,
    #[doc = "South Africa"]
    Za,
    #[doc = "South Georgia and the South Sandwich Islands"]
    Gs,
    #[doc = "South Sudan"]
    Ss,
    #[doc = "Spain"]
    Es,
    #[doc = "Sri Lanka"]
    Lk,
    #[doc = "Sudan"]
    Sd,
    #[doc = "Suriname"]
    Sr,
    #[doc = "Svalbard and Jan Mayen"]
    Sj,
    #[doc = "Swaziland"]
    Sz,
    #[doc = "Sweden"]
    Se,
    #[doc = "Switzerland"]
    Ch,
    #[doc = "Syrian Arab Republic"]
    Sy,
    #[doc = "Taiwan, Province of China"]
    Tw,
    #[doc = "Tajikistan"]
    Tj,
    #[doc = "Tanzania, United Republic of"]
    Tz,
    #[doc = "Thailand"]
    Th,
    #[doc = "Timor-Leste"]
    Tl,
    #[doc = "Togo"]
    Tg,
    #[doc = "Tokelau"]
    Tk,
    #[doc = "Tonga"]
    To,
    #[doc = "Trinidad and Tobago"]
    Tt,
    #[doc = "Tunisia"]
    Tn,
    #[doc = "Turkey"]
    Tr,
    #[doc = "Turkmenistan"]
    Tm,
    #[doc = "Turks and Caicos Islands"]
    Tc,
    #[doc = "Tuvalu"]
    Tv,
    #[doc = "Uganda"]
    Ug,
    #[doc = "Ukraine"]
    Ua,
    #[doc = "United Arab Emirates"]
    Ae,
    #[doc = "United Kingdom of Great Britain and Northern Ireland"]
    Gb,
    #[doc = "United States of America"]
    Us,
    #[doc = "United States Minor Outlying Islands"]
    Um,
    #[doc = "Uruguay"]
    Uy,
    #[doc = "Uzbekistan"]
    Uz,
    #[doc = "Vanuatu"]
    Vu,
    #[doc = "Venezuela (Bolivarian Republic of)"]
    Ve,
    #[doc = "Viet Nam"]
    Vn,
    #[doc = "Virgin Islands (British)"]
    Vg,
    #[doc = "Virgin Islands (U.S.)"]
    Vi,
    #[doc = "Wallis and Futuna"]
    Wf,
    #[doc = "Western Sahara"]
    Eh,
    #[doc = "Yemen"]
    Ye,
    #[doc = "Zambia"]
    Zm,
    #[doc = "Zimbabwe"]
    Zw,
}
