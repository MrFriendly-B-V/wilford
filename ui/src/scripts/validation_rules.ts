import {InputValidationRules} from "@/main";

export const REQUIRED_RULES: InputValidationRules = [
  v => !!v || "Required",

]

export const EMAIL_RULES: InputValidationRules = [
  v => !!v || "Required",
]

export const PASSWORD_RULES: InputValidationRules = [
  v => !!v || "Required",
]