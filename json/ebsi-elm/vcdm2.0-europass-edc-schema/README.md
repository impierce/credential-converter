![EBSI Logo](https://ec.europa.eu/digital-building-blocks/sites/images/logo/default-space-logo.svg)

# @cef-ebsi/vcdm2.0-europass-edc-schema

> Europass EDC credential
>
> Schema for EDC credential based on ELM 3.2

The schema is published to the [Trusted Schemas Registry](https://hub.ebsi.eu/apis/pilot/trusted-schemas-registry) with the IDs:

- `0x4d40d34533797f62a8a03d7c4a4cd7827d289fadf1bfa582ec0ecfedbe55a6e0` (hexadecimal)
- `z6CZj2KLNFkFqeDrJ9h27Rgc2RhszhFRHJ5QTJ3W8p6B1` (multibase base58btc)

## Table of Contents

- [JSON Schema](#json-schema)
- [Installation](#installation)
- [Usage](#usage)
- [License](#license)

## JSON Schema

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "Europass EDC credential",
  "description": "Schema for EDC credential based on ELM 3.2",
  "type": "object",
  "allOf": [
    {
      "$ref": "./node_modules/@cef-ebsi/vcdm2.0-attestation-schema/schema.json"
    },
    {
      "$ref": "#/$defs/EuropeanDigitalCredentialType"
    }
  ],
  "$defs": {
    "CredentialSubjectType": {
      "$ref": "#/$defs/AgentOrPersonOrOrganisationType"
    },
    "IntegerType": {
      "type": "integer"
    },
    "PositiveIntegerType": {
      "type": "integer",
      "minimum": 0
    },
    "PercentageIntegerType": {
      "type": "integer",
      "minimum": 0,
      "maximum": 100
    },
    "DecimalType": {
      "type": "number"
    },
    "BooleanType": {
      "type": "boolean"
    },
    "IRIType": {
      "type": "string"
    },
    "URIType": {
      "type": "string",
      "format": "uri"
    },
    "Many!HTMLType": {
      "anyOf": [
        {
          "$ref": "#/$defs/HTMLType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/HTMLType"
          }
        }
      ]
    },
    "HTMLType": {
      "type": "string"
    },
    "DateTimeType": {
      "type": "string",
      "format": "date-time"
    },
    "EmailType": {
      "type": "string",
      "anyOf": [
        {
          "format": "email"
        },
        {
          "format": "uri",
          "pattern": "^mailto:[^@]*[^\\.]@[^\\.]($|[^@]*[^\\.]$)"
        }
      ]
    },
    "DurationType": {
      "type": "string",
      "format": "duration"
    },
    "Many!PeriodOfTimeType": {
      "anyOf": [
        {
          "$ref": "#/$defs/PeriodOfTimeType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PeriodOfTimeType"
          }
        }
      ]
    },
    "PeriodOfTimeType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "PeriodOfTime"
        },
        "startDate": {
          "$ref": "#/$defs/DateTimeType"
        },
        "endDate": {
          "$ref": "#/$defs/DateTimeType"
        },
        "prefLabel": {
          "$ref": "#/$defs/Many!LangStringType"
        }
      },
      "required": []
    },
    "Many!StringType": {
      "anyOf": [
        {
          "$ref": "#/$defs/StringType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/StringType"
          }
        }
      ]
    },
    "StringType": {
      "type": "string"
    },
    "GenericIdType": {
      "$ref": "#/$defs/URIType"
    },
    "LiteralType": {
      "$ref": "#/$defs/StringType"
    },
    "Many!AgentType": {
      "anyOf": [
        {
          "$ref": "#/$defs/AgentType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/AgentType"
          }
        }
      ]
    },
    "AgentType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "Agent"
        },
        "identifier": {
          "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
        },
        "altLabel": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "prefLabel": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "location": {
          "$ref": "#/$defs/Many!LocationType"
        },
        "contactPoint": {
          "$ref": "#/$defs/Many!ContactPointType"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "groupMemberOf": {
          "$ref": "#/$defs/Many!GroupType"
        },
        "dateModified": {
          "$ref": "#/$defs/DateTimeType"
        }
      },
      "required": []
    },
    "Many!PersonType": {
      "anyOf": [
        {
          "$ref": "#/$defs/PersonType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PersonType"
          }
        }
      ]
    },
    "PersonType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "Person"
        },
        "identifier": {
          "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
        },
        "location": {
          "$ref": "#/$defs/LocationType"
        },
        "nationalID": {
          "$ref": "#/$defs/LegalIdentifierType"
        },
        "fullName": {
          "$ref": "#/$defs/LangStringType"
        },
        "givenName": {
          "$ref": "#/$defs/LangStringType"
        },
        "familyName": {
          "$ref": "#/$defs/LangStringType"
        },
        "birthName": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "patronymicName": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "memberOf": {
          "$ref": "#/$defs/Many!OrganisationType"
        },
        "dateOfBirth": {
          "$ref": "#/$defs/DateTimeType"
        },
        "placeOfBirth": {
          "$ref": "#/$defs/LocationType"
        },
        "citizenshipCountry": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "gender": {
          "$ref": "#/$defs/ConceptType"
        },
        "contactPoint": {
          "$ref": "#/$defs/Many!ContactPointType"
        },
        "groupMemberOf": {
          "$ref": "#/$defs/Many!GroupType"
        },
        "dateModified": {
          "$ref": "#/$defs/DateTimeType"
        },
        "hasCredential": {
          "$ref": "#/$defs/Many!EuropeanDigitalCredentialType"
        },
        "hasClaim": {
          "$ref": "#/$defs/Many!ClaimNodeType"
        }
      },
      "required": []
    },
    "Many!EuropeanDigitalCredentialType": {
      "anyOf": [
        {
          "$ref": "#/$defs/EuropeanDigitalCredentialType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/EuropeanDigitalCredentialType"
          }
        }
      ]
    },
    "Many!ClaimNodeType": {
      "anyOf": [
        {
          "$ref": "#/$defs/ClaimNodeType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/ClaimNodeType"
          }
        }
      ]
    },
    "ClaimNodeType": {
      "anyOf": [
        {
          "$ref": "#/$defs/LearningAchievementType"
        },
        {
          "$ref": "#/$defs/LearningActivityType"
        },
        {
          "$ref": "#/$defs/LearningAssessmentType"
        },
        {
          "$ref": "#/$defs/LearningEntitlementType"
        },
        {
          "$ref": "#/$defs/ClaimTypeNodeType"
        }
      ]
    },
    "Many!OrganisationType": {
      "anyOf": [
        {
          "$ref": "#/$defs/OrganisationType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/OrganisationType"
          }
        }
      ]
    },
    "OrganisationType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "Organisation"
        },
        "dcType": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "identifier": {
          "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
        },
        "altLabel": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "homepage": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "location": {
          "$ref": "#/$defs/Many!LocationType"
        },
        "accreditation": {
          "$ref": "#/$defs/Many!AccreditationType"
        },
        "eIDASIdentifier": {
          "$ref": "#/$defs/LegalIdentifierType"
        },
        "registration": {
          "$ref": "#/$defs/LegalIdentifierType"
        },
        "legalName": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "vatIdentifier": {
          "$ref": "#/$defs/Many!LegalIdentifierType"
        },
        "taxIdentifier": {
          "$ref": "#/$defs/Many!LegalIdentifierType"
        },
        "logo": {
          "$ref": "#/$defs/MediaObjectType"
        },
        "hasSubOrganization": {
          "$ref": "#/$defs/Many!OrganisationType"
        },
        "subOrganizationOf": {
          "$ref": "#/$defs/OrganisationType"
        },
        "hasMember": {
          "$ref": "#/$defs/Many!PersonType"
        },
        "groupMemberOf": {
          "$ref": "#/$defs/Many!GroupType"
        },
        "contactPoint": {
          "$ref": "#/$defs/Many!ContactPointType"
        },
        "dateModified": {
          "$ref": "#/$defs/DateTimeType"
        }
      },
      "required": ["legalName", "location"]
    },
    "MediaObjectType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "MediaObject"
        },
        "title": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "contentType": {
          "$ref": "#/$defs/ConceptType"
        },
        "attachmentType": {
          "$ref": "#/$defs/ConceptType"
        },
        "contentEncoding": {
          "$ref": "#/$defs/ConceptType"
        },
        "contentSize": {
          "$ref": "#/$defs/IntegerType"
        },
        "content": {
          "$ref": "#/$defs/StringType"
        },
        "contentURL": {
          "$ref": "#/$defs/URIType"
        }
      },
      "required": ["contentType", "contentEncoding", "content"]
    },
    "Many!AccreditationType": {
      "anyOf": [
        {
          "$ref": "#/$defs/AccreditationType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/AccreditationType"
          }
        }
      ]
    },
    "Many!IssuerNodeType": {
      "anyOf": [
        {
          "$ref": "#/$defs/IssuerNodeType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/IssuerNodeType"
          }
        }
      ]
    },
    "IssuerNodeType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "IssuerNode"
        },
        "eidasLegalIdentifier": {
          "$ref": "#/$defs/LegalIdentifierType"
        }
      },
      "required": ["eidasLegalIdentifier"]
    },
    "AccreditationType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "Accreditation"
        },
        "dcType": {
          "$ref": "#/$defs/ConceptType"
        },
        "identifier": {
          "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
        },
        "title": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "homepage": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "dateIssued": {
          "$ref": "#/$defs/DateTimeType"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "supplementaryDocument": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "decision": {
          "$ref": "#/$defs/ConceptType"
        },
        "report": {
          "$ref": "#/$defs/WebResourceType"
        },
        "organisation": {
          "$ref": "#/$defs/Many!OrganisationType"
        },
        "limitQualification": {
          "$ref": "#/$defs/QualificationType"
        },
        "limitField": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "limitEQFLevel": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "limitJurisdiction": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "limitCredentialType": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "accreditingAgent": {
          "$ref": "#/$defs/OrganisationType"
        },
        "reviewDate": {
          "$ref": "#/$defs/DateTimeType"
        },
        "expiryDate": {
          "$ref": "#/$defs/DateTimeType"
        },
        "landingPage": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "status": {
          "$ref": "#/$defs/StringType"
        },
        "dateModified": {
          "$ref": "#/$defs/DateTimeType"
        }
      },
      "required": ["title", "accreditingAgent", "dcType"]
    },
    "Many!QualificationType": {
      "anyOf": [
        {
          "$ref": "#/$defs/QualificationType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/QualificationType"
          }
        }
      ]
    },
    "QualificationType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "Qualification"
        },
        "dcType": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "identifier": {
          "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
        },
        "title": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "supplementaryDocument": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "homepage": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "altLabel": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "category": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "dateModified": {
          "$ref": "#/$defs/DateTimeType"
        },
        "language": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "volumeOfLearning": {
          "$ref": "#/$defs/DurationType"
        },
        "mode": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "learningOutcomeSummary": {
          "$ref": "#/$defs/NoteType"
        },
        "thematicArea": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "educationSubject": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "creditPoint": {
          "$ref": "#/$defs/Many!CreditPointType"
        },
        "educationLevel": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "learningSetting": {
          "$ref": "#/$defs/ConceptType"
        },
        "maximumDuration": {
          "$ref": "#/$defs/DurationType"
        },
        "targetGroup": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "entryRequirement": {
          "$ref": "#/$defs/NoteType"
        },
        "learningOutcome": {
          "$ref": "#/$defs/Many!LearningOutcomeType"
        },
        "influencedBy": {
          "$ref": "#/$defs/Many!LearningActivitySpecificationType"
        },
        "provenBy": {
          "$ref": "#/$defs/Many!LearningAssessmentSpecificationType"
        },
        "entitlesTo": {
          "$ref": "#/$defs/Many!LearningEntitlementSpecificationType"
        },
        "awardingOpportunity": {
          "$ref": "#/$defs/Many!AwardingOpportunityType"
        },
        "hasPart": {
          "$ref": "#/$defs/Many!QualificationType"
        },
        "isPartOf": {
          "$ref": "#/$defs/Many!QualificationType"
        },
        "specialisationOf": {
          "$ref": "#/$defs/Many!QualificationType"
        },
        "generalisationOf": {
          "$ref": "#/$defs/Many!QualificationType"
        },
        "isPartialQualification": {
          "$ref": "#/$defs/BooleanType"
        },
        "eqfLevel": {
          "$ref": "#/$defs/ConceptType"
        },
        "nqfLevel": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "accreditation": {
          "$ref": "#/$defs/Many!AccreditationType"
        },
        "qualificationCode": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "status": {
          "$ref": "#/$defs/StringType"
        }
      },
      "required": ["title"]
    },
    "Many!LearningOutcomeType": {
      "anyOf": [
        {
          "$ref": "#/$defs/LearningOutcomeType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/LearningOutcomeType"
          }
        }
      ]
    },
    "LearningOutcomeType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "LearningOutcome"
        },
        "dcType": {
          "$ref": "#/$defs/ConceptType"
        },
        "identifier": {
          "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
        },
        "title": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "reusabilityLevel": {
          "$ref": "#/$defs/ConceptType"
        },
        "relatedSkill": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "relatedESCOSkill": {
          "$ref": "#/$defs/Many!ConceptType"
        }
      },
      "required": ["title"]
    },
    "Many!ContactPointType": {
      "anyOf": [
        {
          "$ref": "#/$defs/ContactPointType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/ContactPointType"
          }
        }
      ]
    },
    "ContactPointType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "ContactPoint"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "address": {
          "$ref": "#/$defs/Many!AddressType"
        },
        "phone": {
          "$ref": "#/$defs/Many!PhoneType"
        },
        "emailAddress": {
          "$ref": "#/$defs/Many!MailboxType"
        },
        "contactForm": {
          "$ref": "#/$defs/Many!WebResourceType"
        }
      },
      "required": []
    },
    "Many!NoteType": {
      "anyOf": [
        {
          "$ref": "#/$defs/NoteType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/NoteType"
          }
        }
      ]
    },
    "NoteType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "Note"
        },
        "noteLiteral": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "subject": {
          "$ref": "#/$defs/ConceptType"
        },
        "noteFormat": {
          "$ref": "#/$defs/ConceptType"
        }
      },
      "required": ["noteLiteral"]
    },
    "Many!AddressType": {
      "anyOf": [
        {
          "$ref": "#/$defs/AddressType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/AddressType"
          }
        }
      ]
    },
    "AddressType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "Address"
        },
        "identifier": {
          "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
        },
        "fullAddress": {
          "$ref": "#/$defs/NoteType"
        },
        "countryCode": {
          "$ref": "#/$defs/ConceptType"
        }
      },
      "required": ["countryCode"]
    },
    "Many!PhoneType": {
      "anyOf": [
        {
          "$ref": "#/$defs/PhoneType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PhoneType"
          }
        }
      ]
    },
    "PhoneType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "Phone"
        },
        "phoneNumber": {
          "$ref": "#/$defs/StringType"
        },
        "countryDialing": {
          "$ref": "#/$defs/StringType"
        },
        "areaDialing": {
          "$ref": "#/$defs/StringType"
        },
        "dialNumber": {
          "$ref": "#/$defs/StringType"
        }
      },
      "required": []
    },
    "Many!MailboxType": {
      "anyOf": [
        {
          "$ref": "#/$defs/MailboxType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/MailboxType"
          }
        }
      ]
    },
    "MailboxType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/EmailType"
        },
        "type": {
          "const": "Mailbox"
        }
      },
      "required": []
    },
    "Many!WebResourceType": {
      "anyOf": [
        {
          "$ref": "#/$defs/WebResourceType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/WebResourceType"
          }
        }
      ]
    },
    "WebResourceType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "WebResource"
        },
        "title": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "language": {
          "$ref": "#/$defs/ConceptType"
        },
        "contentURL": {
          "$ref": "#/$defs/URIType"
        }
      },
      "required": ["contentURL"]
    },
    "Many!ConceptType": {
      "anyOf": [
        {
          "$ref": "#/$defs/ConceptType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/ConceptType"
          }
        }
      ]
    },
    "Single!ConceptType": {
      "anyOf": [
        {
          "$ref": "#/$defs/ConceptType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/ConceptType"
          },
          "minItems": 1,
          "maxItems": 1
        }
      ]
    },
    "ConceptType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "Concept"
        },
        "prefLabel": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "notation": {
          "$ref": "#/$defs/LiteralType"
        },
        "inScheme": {
          "$ref": "#/$defs/ConceptSchemeType"
        },
        "definition": {
          "$ref": "#/$defs/Many!LangStringType"
        }
      },
      "required": []
    },
    "ConceptSchemeType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "ConceptScheme"
        }
      },
      "required": []
    },
    "Many!LocationType": {
      "anyOf": [
        {
          "$ref": "#/$defs/LocationType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/LocationType"
          }
        }
      ]
    },
    "LocationType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "Location"
        },
        "identifier": {
          "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "address": {
          "$ref": "#/$defs/Many!AddressType"
        },
        "geographicName": {
          "$ref": "#/$defs/Many!AddressType"
        },
        "spatialCode": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "geometry": {
          "$ref": "#/$defs/Many!GeometryType"
        }
      },
      "required": ["address"]
    },
    "Many!GeometryType": {
      "anyOf": [
        {
          "$ref": "#/$defs/GeometryType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/GeometryType"
          }
        }
      ]
    },
    "GeometryType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "Geometry"
        },
        "longitude": {
          "$ref": "#/$defs/StringType"
        },
        "latitude": {
          "$ref": "#/$defs/StringType"
        }
      },
      "required": []
    },
    "Many!GroupType": {
      "anyOf": [
        {
          "$ref": "#/$defs/GroupType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/GroupType"
          }
        }
      ]
    },
    "GroupType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "Group"
        },
        "prefLabel": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "altLabel": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "location": {
          "$ref": "#/$defs/Many!LocationType"
        },
        "contactPoint": {
          "$ref": "#/$defs/Many!ContactPointType"
        },
        "member": {
          "$ref": "#/$defs/Many!AgentOrPersonOrOrganisationType"
        }
      },
      "required": ["prefLabel"]
    },
    "Many!AgentOrPersonOrOrganisationType": {
      "anyOf": [
        {
          "$ref": "#/$defs/AgentOrPersonOrOrganisationType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/AgentOrPersonOrOrganisationType"
          }
        }
      ]
    },
    "AgentOrPersonOrOrganisationType": {
      "anyOf": [
        {
          "$ref": "#/$defs/AgentType"
        },
        {
          "$ref": "#/$defs/PersonType"
        },
        {
          "$ref": "#/$defs/OrganisationType"
        }
      ]
    },
    "LearningAchievementSpecificationOrSpecificationType": {
      "anyOf": [
        {
          "$ref": "#/$defs/LearningAchievementSpecificationType"
        },
        {
          "$ref": "#/$defs/QualificationType"
        }
      ]
    },
    "IdentifierOrLegalIdentifierType": {
      "anyOf": [
        {
          "$ref": "#/$defs/IdentifierType"
        },
        {
          "$ref": "#/$defs/LegalIdentifierType"
        }
      ]
    },
    "Many!IdentifierType": {
      "anyOf": [
        {
          "$ref": "#/$defs/IdentifierType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/IdentifierType"
          }
        }
      ]
    },
    "IdentifierType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "Identifier"
        },
        "dcType": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "notation": {
          "$ref": "#/$defs/LiteralType"
        },
        "schemeAgency": {
          "$ref": "#/$defs/LangStringType"
        },
        "creator": {
          "$ref": "#/$defs/IRIType"
        },
        "dateIssued": {
          "$ref": "#/$defs/DateTimeType"
        },
        "schemeName": {
          "$ref": "#/$defs/StringType"
        },
        "schemeVersion": {
          "$ref": "#/$defs/StringType"
        },
        "schemeId": {
          "$ref": "#/$defs/URIType"
        }
      },
      "required": ["notation"]
    },
    "Many!LegalIdentifierType": {
      "anyOf": [
        {
          "$ref": "#/$defs/LegalIdentifierType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/LegalIdentifierType"
          }
        }
      ]
    },
    "LegalIdentifierType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "LegalIdentifier"
        },
        "dcType": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "notation": {
          "$ref": "#/$defs/LiteralType"
        },
        "schemeAgency": {
          "$ref": "#/$defs/LangStringType"
        },
        "creator": {
          "$ref": "#/$defs/IRIType"
        },
        "dateIssued": {
          "$ref": "#/$defs/DateTimeType"
        },
        "schemeName": {
          "$ref": "#/$defs/StringType"
        },
        "schemeVersion": {
          "$ref": "#/$defs/StringType"
        },
        "schemeId": {
          "$ref": "#/$defs/URIType"
        },
        "spatial": {
          "$ref": "#/$defs/ConceptType"
        }
      },
      "required": ["notation", "spatial"]
    },
    "Many!CreditPointType": {
      "anyOf": [
        {
          "$ref": "#/$defs/CreditPointType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/CreditPointType"
          }
        }
      ]
    },
    "CreditPointType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "CreditPoint"
        },
        "framework": {
          "$ref": "#/$defs/ConceptType"
        },
        "point": {
          "$ref": "#/$defs/StringType"
        }
      },
      "required": ["framework", "point"]
    },
    "AmountType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "Amount"
        },
        "unit": {
          "$ref": "#/$defs/ConceptType"
        },
        "value": {
          "$ref": "#/$defs/DecimalType"
        }
      },
      "required": ["unit", "value"]
    },
    "Many!LangStringType": {
      "type": "object",
      "propertyNames": {
        "pattern": "^(aa|ab|ae|af|ak|am|an|ar|as|av|ay|az|ba|be|bg|bh|bi|bm|bn|bo|br|bs|ca|ce|ch|co|cr|cs|cu|cv|cy|da|de|dv|dz|ee|el|en|eo|es|et|eu|fa|ff|fi|fj|fo|fr|fy|ga|gd|gl|gn|gu|gv|ha|he|hi|ho|hr|ht|hu|hy|hz|ia|id|ie|ig|ii|ik|in|io|is|it|iu|iw|ja|ji|jv|jw|ka|kg|ki|kj|kk|kl|km|kn|ko|kr|ks|ku|kv|kw|ky|la|lb|lg|li|ln|lo|lt|lu|lv|mg|mh|mi|mk|ml|mn|mo|mr|ms|mt|my|na|nb|nd|ne|ng|nl|nn|no|nr|nv|ny|oc|oj|om|or|os|pa|pi|pl|ps|pt|qu|rm|rn|ro|ru|rw|sa|sc|sd|se|sg|sh|si|sk|sl|sm|sn|so|sq|sr|ss|st|su|sv|sw|ta|te|tg|th|ti|tk|tl|tn|to|tr|ts|tt|tw|ty|ug|uk|ur|uz|ve|vi|vo|wa|wo|xh|yi|yo|za|zh|zu)$"
      },
      "minProperties": 1
    },
    "LangStringType": {
      "allOf": [
        {
          "$ref": "#/$defs/Many!LangStringType"
        },
        {
          "type": "object",
          "maxProperties": 1
        }
      ]
    },
    "Many!LearningAchievementType": {
      "anyOf": [
        {
          "$ref": "#/$defs/LearningAchievementType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/LearningAchievementType"
          }
        }
      ]
    },
    "LearningAchievementType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "LearningAchievement"
        },
        "dcType": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "title": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "identifier": {
          "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "supplementaryDocument": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "learningOpportunity": {
          "$ref": "#/$defs/LearningOpportunityType"
        },
        "creditReceived": {
          "$ref": "#/$defs/Many!CreditPointType"
        },
        "provenBy": {
          "$ref": "#/$defs/Many!LearningAssessmentType"
        },
        "influencedBy": {
          "$ref": "#/$defs/Many!LearningActivityType"
        },
        "awardedBy": {
          "$ref": "#/$defs/AwardingProcessType"
        },
        "entitlesTo": {
          "$ref": "#/$defs/Many!LearningEntitlementType"
        },
        "specifiedBy": {
          "$ref": "#/$defs/LearningAchievementSpecificationOrQualificationType"
        },
        "hasPart": {
          "$ref": "#/$defs/Many!LearningAchievementType"
        },
        "isPartOf": {
          "$ref": "#/$defs/Many!LearningAchievementType"
        }
      },
      "required": ["title", "awardedBy"]
    },
    "Many!LearningAchievementSpecificationType": {
      "anyOf": [
        {
          "$ref": "#/$defs/LearningAchievementSpecificationType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/LearningAchievementSpecificationType"
          }
        }
      ]
    },
    "LearningAchievementSpecificationType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "LearningAchievementSpecification"
        },
        "dcType": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "identifier": {
          "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
        },
        "title": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "supplementaryDocument": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "homepage": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "altLabel": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "category": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "dateModified": {
          "$ref": "#/$defs/DateTimeType"
        },
        "language": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "volumeOfLearning": {
          "$ref": "#/$defs/DurationType"
        },
        "mode": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "learningOutcomeSummary": {
          "$ref": "#/$defs/NoteType"
        },
        "thematicArea": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "educationSubject": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "creditPoint": {
          "$ref": "#/$defs/Many!CreditPointType"
        },
        "educationLevel": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "learningSetting": {
          "$ref": "#/$defs/ConceptType"
        },
        "maximumDuration": {
          "$ref": "#/$defs/DurationType"
        },
        "targetGroup": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "entryRequirement": {
          "$ref": "#/$defs/NoteType"
        },
        "learningOutcome": {
          "$ref": "#/$defs/Many!LearningOutcomeType"
        },
        "influencedBy": {
          "$ref": "#/$defs/Many!LearningActivitySpecificationType"
        },
        "provenBy": {
          "$ref": "#/$defs/Many!LearningAssessmentSpecificationType"
        },
        "entitlesTo": {
          "$ref": "#/$defs/Many!LearningEntitlementSpecificationType"
        },
        "awardingOpportunity": {
          "$ref": "#/$defs/Many!AwardingOpportunityType"
        },
        "hasPart": {
          "$ref": "#/$defs/Many!LearningAchievementSpecificationOrQualificationType"
        },
        "isPartOf": {
          "$ref": "#/$defs/Many!LearningAchievementSpecificationOrQualificationType"
        },
        "specialisationOf": {
          "$ref": "#/$defs/Many!LearningAchievementSpecificationOrQualificationType"
        },
        "generalisationOf": {
          "$ref": "#/$defs/Many!LearningAchievementSpecificationOrQualificationType"
        },
        "status": {
          "$ref": "#/$defs/StringType"
        }
      },
      "required": ["title"]
    },
    "Many!LearningActivityType": {
      "anyOf": [
        {
          "$ref": "#/$defs/LearningActivityType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/LearningActivityType"
          }
        }
      ]
    },
    "LearningActivityType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "LearningActivity"
        },
        "dcType": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "title": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "identifier": {
          "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "supplementaryDocument": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "temporal": {
          "$ref": "#/$defs/Many!PeriodOfTimeType"
        },
        "location": {
          "$ref": "#/$defs/Many!LocationType"
        },
        "learningOpportunity": {
          "$ref": "#/$defs/LearningOpportunityType"
        },
        "workload": {
          "$ref": "#/$defs/DurationType"
        },
        "directedBy": {
          "$ref": "#/$defs/Many!AgentOrPersonOrOrganisationType"
        },
        "awardedBy": {
          "$ref": "#/$defs/AwardingProcessType"
        },
        "influences": {
          "$ref": "#/$defs/Many!LearningAchievementType"
        },
        "specifiedBy": {
          "$ref": "#/$defs/LearningActivitySpecificationType"
        },
        "hasPart": {
          "$ref": "#/$defs/Many!LearningActivityType"
        },
        "isPartOf": {
          "$ref": "#/$defs/Many!LearningActivityType"
        },
        "levelOfCompletion": {
          "$ref": "#/$defs/PercentageIntegerType"
        }
      },
      "required": ["title", "awardedBy"]
    },
    "Many!LearningActivitySpecificationType": {
      "anyOf": [
        {
          "$ref": "#/$defs/LearningActivitySpecificationType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/LearningActivitySpecificationType"
          }
        }
      ]
    },
    "LearningActivitySpecificationType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "LearningActivitySpecification"
        },
        "dcType": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "identifier": {
          "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
        },
        "title": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "supplementaryDocument": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "homepage": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "altLabel": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "category": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "dateModified": {
          "$ref": "#/$defs/DateTimeType"
        },
        "language": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "volumeOfLearning": {
          "$ref": "#/$defs/DurationType"
        },
        "contactHour": {
          "$ref": "#/$defs/Many!StringType"
        },
        "mode": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "influences": {
          "$ref": "#/$defs/Many!LearningAchievementSpecificationOrQualificationType"
        },
        "hasPart": {
          "$ref": "#/$defs/Many!LearningActivitySpecificationType"
        },
        "isPartOf": {
          "$ref": "#/$defs/Many!LearningActivitySpecificationType"
        },
        "specialisationOf": {
          "$ref": "#/$defs/Many!LearningActivitySpecificationType"
        },
        "generalisationOf": {
          "$ref": "#/$defs/Many!LearningActivitySpecificationType"
        },
        "status": {
          "$ref": "#/$defs/StringType"
        }
      },
      "required": ["title"]
    },
    "Many!LearningAssessmentType": {
      "anyOf": [
        {
          "$ref": "#/$defs/LearningAssessmentType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/LearningAssessmentType"
          }
        }
      ]
    },
    "LearningAssessmentType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "LearningAssessment"
        },
        "dcType": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "title": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "identifier": {
          "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "supplementaryDocument": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "dateIssued": {
          "$ref": "#/$defs/DateTimeType"
        },
        "location": {
          "$ref": "#/$defs/LocationType"
        },
        "grade": {
          "$ref": "#/$defs/NoteType"
        },
        "gradeStatus": {
          "$ref": "#/$defs/ConceptType"
        },
        "shortenedGrading": {
          "$ref": "#/$defs/ShortenedGradingType"
        },
        "resultDistribution": {
          "$ref": "#/$defs/ResultDistributionType"
        },
        "idVerification": {
          "$ref": "#/$defs/ConceptType"
        },
        "awardedBy": {
          "$ref": "#/$defs/AwardingProcessType"
        },
        "assessedBy": {
          "$ref": "#/$defs/Many!AgentOrPersonOrOrganisationType"
        },
        "proves": {
          "$ref": "#/$defs/Many!LearningAchievementType"
        },
        "hasPart": {
          "$ref": "#/$defs/Many!LearningAssessmentType"
        },
        "isPartOf": {
          "$ref": "#/$defs/Many!LearningAssessmentType"
        },
        "specifiedBy": {
          "$ref": "#/$defs/Many!LearningAssessmentSpecificationType"
        }
      },
      "required": ["title", "grade", "awardedBy"]
    },
    "Many!LearningAssessmentSpecificationType": {
      "anyOf": [
        {
          "$ref": "#/$defs/LearningAssessmentSpecificationType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/LearningAssessmentSpecificationType"
          }
        }
      ]
    },
    "LearningAssessmentSpecificationType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "LearningAssessmentSpecification"
        },
        "dcType": {
          "$ref": "#/$defs/ConceptType"
        },
        "identifier": {
          "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
        },
        "title": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "supplementaryDocument": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "homepage": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "altLabel": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "category": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "dateModified": {
          "$ref": "#/$defs/DateTimeType"
        },
        "language": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "mode": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "gradingScheme": {
          "$ref": "#/$defs/GradingSchemeType"
        },
        "proves": {
          "$ref": "#/$defs/Many!LearningAchievementSpecificationOrQualificationType"
        },
        "hasPart": {
          "$ref": "#/$defs/Many!LearningAssessmentSpecificationType"
        },
        "isPartOf": {
          "$ref": "#/$defs/Many!LearningAssessmentSpecificationType"
        },
        "specialisationOf": {
          "$ref": "#/$defs/Many!LearningAssessmentSpecificationType"
        },
        "generalisationOf": {
          "$ref": "#/$defs/Many!LearningAssessmentSpecificationType"
        },
        "status": {
          "$ref": "#/$defs/StringType"
        }
      },
      "required": ["title"]
    },
    "Many!LearningEntitlementType": {
      "anyOf": [
        {
          "$ref": "#/$defs/LearningEntitlementType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/LearningEntitlementType"
          }
        }
      ]
    },
    "LearningEntitlementType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "LearningEntitlement"
        },
        "dcType": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "title": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "identifier": {
          "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "supplementaryDocument": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "dateIssued": {
          "$ref": "#/$defs/DateTimeType"
        },
        "expiryDate": {
          "$ref": "#/$defs/DateTimeType"
        },
        "awardedBy": {
          "$ref": "#/$defs/AwardingProcessType"
        },
        "entitledBy": {
          "$ref": "#/$defs/Many!LearningAchievementType"
        },
        "hasPart": {
          "$ref": "#/$defs/Many!LearningEntitlementType"
        },
        "isPartOf": {
          "$ref": "#/$defs/Many!LearningEntitlementType"
        },
        "specifiedBy": {
          "$ref": "#/$defs/Many!LearningEntitlementSpecificationType"
        }
      },
      "required": ["title", "awardedBy"]
    },
    "Many!LearningEntitlementSpecificationType": {
      "anyOf": [
        {
          "$ref": "#/$defs/LearningEntitlementSpecificationType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/LearningEntitlementSpecificationType"
          }
        }
      ]
    },
    "LearningEntitlementSpecificationType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "LearningEntitlementSpecification"
        },
        "dcType": {
          "$ref": "#/$defs/Single!ConceptType"
        },
        "identifier": {
          "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
        },
        "title": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "supplementaryDocument": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "homepage": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "altLabel": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "category": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "dateModified": {
          "$ref": "#/$defs/DateTimeType"
        },
        "entitlementStatus": {
          "$ref": "#/$defs/ConceptType"
        },
        "limitOrganisation": {
          "$ref": "#/$defs/Many!OrganisationType"
        },
        "limitJurisdiction": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "limitOccupation": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "limitNationalOccupation": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "entitledBy": {
          "$ref": "#/$defs/Many!LearningAchievementSpecificationOrQualificationType"
        },
        "hasPart": {
          "$ref": "#/$defs/Many!LearningEntitlementSpecificationType"
        },
        "isPartOf": {
          "$ref": "#/$defs/Many!LearningEntitlementSpecificationType"
        },
        "specialisationOf": {
          "$ref": "#/$defs/Many!LearningEntitlementSpecificationType"
        },
        "generalisationOf": {
          "$ref": "#/$defs/Many!LearningEntitlementSpecificationType"
        },
        "status": {
          "$ref": "#/$defs/StringType"
        }
      },
      "required": ["title", "entitlementStatus", "dcType"]
    },
    "Many!LearningOpportunityType": {
      "anyOf": [
        {
          "$ref": "#/$defs/LearningOpportunityType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/LearningOpportunityType"
          }
        }
      ]
    },
    "LearningOpportunityType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "LearningOpportunity"
        },
        "dcType": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "identifier": {
          "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
        },
        "title": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "homepage": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "supplementaryDocument": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "temporal": {
          "$ref": "#/$defs/PeriodOfTimeType"
        },
        "duration": {
          "$ref": "#/$defs/DurationType"
        },
        "mode": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "learningSchedule": {
          "$ref": "#/$defs/ConceptType"
        },
        "scheduleInformation": {
          "$ref": "#/$defs/NoteType"
        },
        "admissionProcedure": {
          "$ref": "#/$defs/NoteType"
        },
        "priceDetail": {
          "$ref": "#/$defs/Many!PriceDetailType"
        },
        "providedBy": {
          "$ref": "#/$defs/OrganisationType"
        },
        "grant": {
          "$ref": "#/$defs/Many!GrantType"
        },
        "location": {
          "$ref": "#/$defs/Many!LocationType"
        },
        "learningAchievementSpecification": {
          "$ref": "#/$defs/LearningAchievementSpecificationOrQualificationType"
        },
        "learningActivitySpecification": {
          "$ref": "#/$defs/LearningActivitySpecificationType"
        },
        "hasPart": {
          "$ref": "#/$defs/Many!LearningOpportunityType"
        },
        "isPartOf": {
          "$ref": "#/$defs/Many!LearningOpportunityType"
        },
        "bannerImage": {
          "$ref": "#/$defs/MediaObjectType"
        },
        "applicationDeadline": {
          "$ref": "#/$defs/Many!DateTimeType"
        },
        "defaultLanguage": {
          "$ref": "#/$defs/ConceptType"
        },
        "descriptionHtml": {
          "$ref": "#/$defs/Many!HTMLType"
        },
        "dateModified": {
          "$ref": "#/$defs/DateTimeType"
        },
        "status": {
          "$ref": "#/$defs/StringType"
        }
      },
      "required": ["title"]
    },
    "Many!PriceDetailType": {
      "anyOf": [
        {
          "$ref": "#/$defs/PriceDetailType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PriceDetailType"
          }
        }
      ]
    },
    "PriceDetailType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "PriceDetail"
        },
        "identifier": {
          "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
        },
        "prefLabel": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "amount": {
          "$ref": "#/$defs/AmountType"
        }
      },
      "required": []
    },
    "Many!ResultCategoryType": {
      "anyOf": [
        {
          "$ref": "#/$defs/ResultCategoryType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/ResultCategoryType"
          }
        }
      ]
    },
    "ResultCategoryType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "ResultCategory"
        },
        "label": {
          "$ref": "#/$defs/StringType"
        },
        "score": {
          "$ref": "#/$defs/StringType"
        },
        "maximumScore": {
          "$ref": "#/$defs/StringType"
        },
        "minimumScore": {
          "$ref": "#/$defs/StringType"
        },
        "count": {
          "$ref": "#/$defs/PositiveIntegerType"
        }
      },
      "required": ["label", "count"]
    },
    "Many!ResultDistributionType": {
      "anyOf": [
        {
          "$ref": "#/$defs/ResultDistributionType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/ResultDistributionType"
          }
        }
      ]
    },
    "ResultDistributionType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "ResultDistribution"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "resultCategory": {
          "$ref": "#/$defs/Many!ResultCategoryType"
        }
      },
      "required": []
    },
    "Many!ShortenedGradingType": {
      "anyOf": [
        {
          "$ref": "#/$defs/ShortenedGradingType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/ShortenedGradingType"
          }
        }
      ]
    },
    "ShortenedGradingType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "ShortenedGrading"
        },
        "percentageLower": {
          "$ref": "#/$defs/IntegerType"
        },
        "percentageEqual": {
          "$ref": "#/$defs/IntegerType"
        },
        "percentageHigher": {
          "$ref": "#/$defs/IntegerType"
        }
      },
      "required": ["percentageLower", "percentageEqual", "percentageHigher"]
    },
    "Many!VerificationCheckType": {
      "anyOf": [
        {
          "$ref": "#/$defs/VerificationCheckType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/VerificationCheckType"
          }
        }
      ]
    },
    "VerificationCheckType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "VerificationCheck"
        },
        "dcType": {
          "$ref": "#/$defs/ConceptType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "verificationStatus": {
          "$ref": "#/$defs/ConceptType"
        },
        "elmSubject": {
          "$ref": "#/$defs/EuropeanDigitalCredentialType"
        }
      },
      "required": ["verificationStatus", "subject", "dcType"]
    },
    "Many!EvidenceType": {
      "anyOf": [
        {
          "$ref": "#/$defs/EvidenceType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/EvidenceType"
          }
        }
      ]
    },
    "EvidenceType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "Evidence"
        },
        "evidenceStatement": {
          "$ref": "#/$defs/StringType"
        },
        "evidenceTarget": {
          "$ref": "#/$defs/AgentOrPersonOrOrganisationType"
        },
        "embeddedEvidence": {
          "$ref": "#/$defs/Many!MediaObjectType"
        },
        "accreditation": {
          "$ref": "#/$defs/AccreditationType"
        },
        "dcType": {
          "$ref": "#/$defs/ConceptType"
        }
      },
      "required": []
    },
    "Many!TermsOfUseType": {
      "anyOf": [
        {
          "$ref": "#/$defs/TermsOfUseType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/TermsOfUseType"
          }
        }
      ]
    },
    "TermsOfUseType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "TermsOfUse"
        }
      },
      "required": []
    },
    "Many!ProofType": {
      "anyOf": [
        {
          "$ref": "#/$defs/ProofType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/ProofType"
          }
        }
      ]
    },
    "ProofType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "Proof"
        }
      },
      "required": []
    },
    "Many!CredentialStatusType": {
      "anyOf": [
        {
          "$ref": "#/$defs/CredentialStatusType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/CredentialStatusType"
          }
        }
      ]
    },
    "CredentialStatusType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "type": "string",
          "enum": ["CredentialStatus", "TrustedCredentialStatus2021"]
        }
      },
      "required": []
    },
    "Many!CredentialSchemaType": {
      "anyOf": [
        {
          "$ref": "#/$defs/CredentialSchemaType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/CredentialSchemaType"
          }
        }
      ]
    },
    "CredentialSchemaType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "type": "string",
          "enum": ["ShaclValidator2017", "JsonSchema"]
        }
      },
      "required": []
    },
    "Many!AmountType": {
      "anyOf": [
        {
          "$ref": "#/$defs/AmountType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/AmountType"
          }
        }
      ]
    },
    "Many!AwardingProcessType": {
      "anyOf": [
        {
          "$ref": "#/$defs/AwardingProcessType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/AwardingProcessType"
          }
        }
      ]
    },
    "AwardingProcessType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "AwardingProcess"
        },
        "identifier": {
          "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "location": {
          "$ref": "#/$defs/LocationType"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "used": {
          "$ref": "#/$defs/Many!LearningAssessmentType"
        },
        "awards": {
          "$ref": "#/$defs/Many!ClaimNodeType"
        },
        "awardingBody": {
          "$ref": "#/$defs/Many!AgentOrPersonOrOrganisationType"
        },
        "awardingDate": {
          "$ref": "#/$defs/DateTimeType"
        },
        "educationalSystemNote": {
          "$ref": "#/$defs/ConceptType"
        }
      },
      "required": ["awardingBody"]
    },
    "Many!DisplayParameterType": {
      "anyOf": [
        {
          "$ref": "#/$defs/DisplayParameterType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/DisplayParameterType"
          }
        }
      ]
    },
    "DisplayParameterType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "DisplayParameter"
        },
        "title": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "language": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "primaryLanguage": {
          "$ref": "#/$defs/ConceptType"
        },
        "summaryDisplay": {
          "$ref": "#/$defs/StringType"
        },
        "individualDisplay": {
          "$ref": "#/$defs/Many!IndividualDisplayType"
        }
      },
      "required": ["title", "language", "primaryLanguage", "individualDisplay"]
    },
    "Many!IndividualDisplayType": {
      "anyOf": [
        {
          "$ref": "#/$defs/IndividualDisplayType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/IndividualDisplayType"
          }
        }
      ]
    },
    "IndividualDisplayType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "IndividualDisplay"
        },
        "language": {
          "$ref": "#/$defs/ConceptType"
        },
        "displayDetail": {
          "$ref": "#/$defs/Many!DisplayDetailType"
        }
      },
      "required": ["language", "displayDetail"]
    },
    "Many!DisplayDetailType": {
      "anyOf": [
        {
          "$ref": "#/$defs/DisplayDetailType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/DisplayDetailType"
          }
        }
      ]
    },
    "DisplayDetailType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "DisplayDetail"
        },
        "image": {
          "$ref": "#/$defs/MediaObjectType"
        },
        "page": {
          "$ref": "#/$defs/PositiveIntegerType"
        }
      },
      "required": ["image", "page"]
    },
    "Many!EuropeanDigitalPresentationType": {
      "anyOf": [
        {
          "$ref": "#/$defs/EuropeanDigitalPresentationType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/EuropeanDigitalPresentationType"
          }
        }
      ]
    },
    "EuropeanDigitalPresentationType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "EuropeanDigitalPresentation"
        },
        "verifiableCredential": {
          "$ref": "#/$defs/Many!EuropeanDigitalCredentialType"
        },
        "verificationCheck": {
          "$ref": "#/$defs/Many!VerificationCheckType"
        },
        "holder": {
          "$ref": "#/$defs/Many!AgentOrPersonOrOrganisationType"
        },
        "proof": {
          "$ref": "#/$defs/Many!ProofType"
        }
      },
      "required": []
    },
    "Many!GradingSchemeType": {
      "anyOf": [
        {
          "$ref": "#/$defs/GradingSchemeType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/GradingSchemeType"
          }
        }
      ]
    },
    "GradingSchemeType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "GradingScheme"
        },
        "identifier": {
          "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
        },
        "title": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "supplementaryDocument": {
          "$ref": "#/$defs/Many!WebResourceType"
        }
      },
      "required": ["title"]
    },
    "Many!GrantType": {
      "anyOf": [
        {
          "$ref": "#/$defs/GrantType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/GrantType"
          }
        }
      ]
    },
    "GrantType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "Grant"
        },
        "dcType": {
          "$ref": "#/$defs/ConceptType"
        },
        "title": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "supplementaryDocument": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "contentURL": {
          "$ref": "#/$defs/URIType"
        }
      },
      "required": ["title"]
    },
    "Many!ClaimTypeNodeType": {
      "anyOf": [
        {
          "$ref": "#/$defs/ClaimTypeNodeType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/ClaimTypeNodeType"
          }
        }
      ]
    },
    "ClaimTypeNodeType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "ClaimTypeNode"
        },
        "title": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "description": {
          "$ref": "#/$defs/Many!LangStringType"
        },
        "identifier": {
          "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
        },
        "additionalNote": {
          "$ref": "#/$defs/Many!NoteType"
        },
        "supplementaryDocument": {
          "$ref": "#/$defs/Many!WebResourceType"
        },
        "awardedBy": {
          "$ref": "#/$defs/AwardingProcessType"
        }
      },
      "required": ["title", "awardedBy"]
    },
    "EuropeanDigitalCredentialType": {
      "type": "object",
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "type": "array",
          "items": {
            "type": "string",
            "enum": [
              "VerifiableCredential",
              "VerifiableAttestation",
              "EuropeanDigitalCredential"
            ]
          },
          "minItems": 3,
          "uniqueItems": true
        },
        "identifier": {
          "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
        },
        "credentialProfiles": {
          "$ref": "#/$defs/Many!ConceptType"
        },
        "attachment": {
          "$ref": "#/$defs/Many!MediaObjectType"
        },
        "displayParameter": {
          "$ref": "#/$defs/DisplayParameterType"
        },
        "issuer": {
          "anyOf": [
            {
              "$ref": "#/$defs/AgentOrPersonOrOrganisationType"
            },
            {
              "$ref": "#/$defs/GenericIdType"
            }
          ]
        },
        "credentialSubject": {
          "$ref": "#/$defs/AgentOrPersonOrOrganisationType"
        },
        "issuanceDate": {
          "$ref": "#/$defs/DateTimeType"
        },
        "issued": {
          "$ref": "#/$defs/DateTimeType"
        },
        "validFrom": {
          "$ref": "#/$defs/DateTimeType"
        },
        "expirationDate": {
          "$ref": "#/$defs/Many!DateTimeType"
        },
        "validUntil": {
          "$ref": "#/$defs/DateTimeType"
        },
        "proof": {
          "$ref": "#/$defs/Many!ProofType"
        },
        "evidence": {
          "$ref": "#/$defs/Many!EvidenceType"
        },
        "termsOfUse": {
          "$ref": "#/$defs/Many!TermsOfUseType"
        },
        "credentialSchema": {
          "$ref": "#/$defs/Many!CredentialSchemaType"
        },
        "credentialStatus": {
          "$ref": "#/$defs/Many!CredentialStatusType"
        },
        "holder": {
          "$ref": "#/$defs/Many!AgentOrPersonOrOrganisationType"
        }
      },
      "required": [
        "credentialProfiles",
        "displayParameter",
        "issuer",
        "credentialSubject",
        "issued",
        "validFrom",
        "credentialSchema"
      ]
    },
    "Many!IdentifierOrLegalIdentifierType": {
      "anyOf": [
        {
          "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
          }
        }
      ]
    },
    "Many!MediaObjectType": {
      "anyOf": [
        {
          "$ref": "#/$defs/MediaObjectType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/MediaObjectType"
          }
        }
      ]
    },
    "Many!DateTimeType": {
      "anyOf": [
        {
          "$ref": "#/$defs/DateTimeType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/DateTimeType"
          }
        }
      ]
    },
    "Many!LearningAchievementSpecificationOrQualificationType": {
      "anyOf": [
        {
          "$ref": "#/$defs/LearningAchievementSpecificationOrQualificationType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/LearningAchievementSpecificationOrQualificationType"
          }
        }
      ]
    },
    "LearningAchievementSpecificationOrQualificationType": {
      "anyOf": [
        {
          "$ref": "#/$defs/LearningAchievementSpecificationType"
        },
        {
          "$ref": "#/$defs/QualificationType"
        }
      ]
    },
    "Many!AwardingOpportunityType": {
      "anyOf": [
        {
          "$ref": "#/$defs/AwardingOpportunityType"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/$defs/AwardingOpportunityType"
          }
        }
      ]
    },
    "AwardingOpportunityType": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/$defs/GenericIdType"
        },
        "type": {
          "const": "AwardingOpportunity"
        },
        "identifier": {
          "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
        },
        "location": {
          "$ref": "#/$defs/LocationType"
        },
        "temporal": {
          "$ref": "#/$defs/PeriodOfTimeType"
        },
        "awardingBody": {
          "$ref": "#/$defs/Many!AgentOrPersonOrOrganisationType"
        },
        "learningAchievementSpecification": {
          "$ref": "#/$defs/LearningAchievementSpecificationOrQualificationType"
        }
      },
      "required": ["awardingBody"]
    }
  }
}
```

## Installation

```sh
# with npm
npm add @cef-ebsi/vcdm2.0-europass-edc-schema@2.1.0

# with Yarn
yarn add @cef-ebsi/vcdm2.0-europass-edc-schema@2.1.0

# with pnpm
pnpm add @cef-ebsi/vcdm2.0-europass-edc-schema@2.1.0
```

## Usage

The package exports the schema and its metadata as JavaScript objects:

```js
import { schema, metadata } from "@cef-ebsi/vcdm2.0-europass-edc-schema";

// you can now use the schema and metadata
```

In addition, the package exports a TypeScript type corresponding to the schema:

```ts
import type { EuropassEDCCredential } from "@cef-ebsi/vcdm2.0-europass-edc-schema";
```

## License

Copyright (c) 2019 European Commission
Licensed under the EUPL, Version 1.2 or - as soon they will be approved by the European Commission - subsequent versions of the EUPL (the "Licence");
You may not use this work except in compliance with the Licence.
You may obtain a copy of the Licence at:

- <https://joinup.ec.europa.eu/page/eupl-text-11-12>

Unless required by applicable law or agreed to in writing, software distributed under the Licence is distributed on an "AS IS" basis, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the Licence for the specific language governing permissions and limitations under the Licence.