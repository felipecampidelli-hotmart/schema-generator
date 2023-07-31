# Documentação
## schema/doc/1.1/mem_community_consume/{entity}/{action}/event-1.0.doc.md
---
#### **DESCRIÇÃO**:
Evento criado quando o usuário recebe o resultado do clique no botão deletar comunidade na visão de consumo

#### **REPOSITÓRIO DO PROJETO:**

[Hotmart App - Android](https://github.com/Hotmart-Org/sparkle-android)

[Hotmart App - iOS](https://github.com/Hotmart-Org/sparkle-ios)

[Microfront Community - Web](https://github.com/Hotmart-Org/app-membership-consume)

#### **Time - Squad**
<Insert squad name>

#### Campos
Nome            | Tipo    | Obrigatório    | Descrição
----------------|---------|----------------|----------
platform        | enum    | sim            | Plataforma em que a comunidade foi criada. Ex: IOS, ANDROID, WEB.
platformDetail  | string  | sim            | Detalhes da plataforma em que a comunidade foi criada. Ex: iPhone 12, Samsung Galaxy S20, Chrome.
appVersion      | string  | sim            | Versão do aplicativo em que a comunidade foi criada. Ex: 1.0.0.
osVersion       | string  | sim            | Versão do sistema operacional em que a comunidade foi criada. Ex: 14.4.
marketplaceId   | integer | sim            | ID do usuário no marketplace. Essa informação é encontrada como id do usuário no Club ou o idSecurity no Hotmart app
ucode           | string  | sim            | Código itendificador do consumidor
userName        | string  | sim            | Username do usuário no Hotmart App
userRole        | enum    | sim            | Código itendificador do consumidor. Ex: OWNER, VIEWER.
mobileUserId    | integer | não            | Id do usuário no Hotmart App
membershipSlug  | string  | sim            | Slug do membership
membershipName  | string  | sim            | Nome do membership
screenName      | string  | sim            | Nome da tela que disparou o evento
eventName       | string  | sim            | Nome do evento. Ex: Evento para rastrear x
