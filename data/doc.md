# Documentação
## schema/doc/1.1/{system}/{entity}/{action}/event-1.0.doc.md
---
#### **DESCRIÇÃO**:
<Insert description>

#### **REPOSITÓRIO DO PROJETO:**

[Hotmart App - Android](https://github.com/Hotmart-Org/sparkle-android)

[Hotmart App - iOS](https://github.com/Hotmart-Org/sparkle-ios)

[Microfront Community - Web](https://github.com/Hotmart-Org/app-membership-moderation-support)

#### **Time - Squad**
<Insert squad name>

#### Campos
Nome            | Tipo    | Obrigatório    | Descrição
----------------|---------|----------------|----------
platform        | enum    | sim            | Plataforma em que a comunidade foi criada. Ex: IOS, ANDROID, WEB.
platformDetail  | string  | sim            | Detalhes da plataforma em que a comunidade foi criada. Ex: iPhone 12, Samsung Galaxy S20, Chrome.
appVersion      | string  | sim            | Versão do aplicativo em que a comunidade foi criada. Ex: 1.0.0.
osVersion       | string  | sim            | Versão do sistema operacional em que a comunidade foi criada. Ex: 14.4.
ucode           | string  | sim            | Código itendificador do consumidor
userId          | integer | sim            | ID do usuário (marketplaceId)
userRole        | string  | sim            | Código itendificador do consumidor. Ex: OWNER, VIEWER.
membershipId    | integer | sim            | ID do membership
membershipSlug  | string  | sim            | Slug do membership
clubVersion     | enum    | sim            | Versão do club de origem do evento (novo ou antigo)
clubId          | integer | não            | ID do usuário caso seja visitante
isLogged        | boolean | sim            | Usuário está logado ou não
screenName      | string  | sim            | Nome da tela que disparou o evento
eventName       | string  | sim            | Nome do evento. Ex: Evento para rastrear x
