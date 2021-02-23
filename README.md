# Muninn - Secure Communication Transfer Protocol

Muninn attempts to provide a modern, secure, communication protocol that can efficiently transfer long-format messages between systems and consumers.

## Motiviation

The primary motivation of Muninn is to act as an eventual replacement for SMTP and the E-Mail family of protocols (POP3, IMAP, etc.).
The reasoning is that SMTP was founded in 1982 (RFC 821) with no anticipation for its wide adoption and use in the future.
Because of this, very little to no security concerns where put in place. And since its inception, additional protocols and features
have been loosely strapped on-top to correct some of these issues. Unfortunatly, with it's backwards compatibility as a necessity
some servers have chosen which features they wish to adopt, and many other services have sacrificed security for adoptability.
Not only have these issues provided major security vulnerabilities, but the general openess of e-mail has made SPAM a bonified term
(and a hated one at that) as well as phishing attacks a norm.

With the advent of new devices and sociatal concerns, Muninn will attempt to modernize as well as secure the future of long-format digital communication.
By long format, thread-based retained messaging is implied. Muninn **does not** aim to tackle real-time messaging in a short-format chat style, plenty
of existing protocols exist for that. Muninn also **does not** tackle real-time multimedia communication such as voice/video services.

## Features & Roadmap

Muninn's initial roadmap is to offer feature-parity with SMTP, POP3, IMAP, and all of their secure counterparts (TLS/SSL).
Development's goal is to offer a suite of publications and software to enable adoption of Muninn.

### Softwware Suite

- A clear and detailed RFC describing the protocol in detail for outside collaboration.
- Provide a C-compatible library for future projects to use, increasing adoptability for third-party products.
- Official binary package for running a server on Windows/Linux machines, providing a running example.
  - Potential SMTP/POP3/IMAP service available for transitioning to Muninn in a single package.
- Client side application for sending/receiving messages, as a publicly available Muninn service.

### Protocol Goals

- Future-thinking and extendable without sacrificing security vulnerabilities.
- Build on-top of newest TLS over TCP security recommendations.
- Satisfy the OWASP-10 where applicable to both server and client applications.
- Ease services aquiring certifications/compliance for private and governmental security and privacy policies, such as: GDPR, ISO, HIPAA, CCPA, NIST, FISMA, etc.
- Cross-platform implementable between Windows, Linux, Mac, iOS, Android as well as embedded platforms where applicable.
- Encryption for "at-rest" and "in-transit" as the default, using modern cryptography standards.
- Interactability with both traditional centralized systems, and decentralized systems (ex. Matrix).
- Optional usage of blockchain techniques for validating and recording messages.
- Opt-in message signing using asymetric encryption keys (ie. PGP).
- Drop-in replacement for E-Mails authentication purposes (account verification, identity, etc.) while provided service providers better usability.
- Enable Hashcash-style proof-of-work system to cut-down on mass/SPAM messages. But also, allowing for categorical newsletters a single hashing method.
- TLS Certificates require third-party vouching such as HTTPS' SSL Certificate system. This validation extends to the users/accounts within that system.

## About The Name

Huginn and Muninn are Odin's ravens in norse mythology. He sends them out to fly all over Midgard and collect information and bring it back to him.
In Old Norse, Huginn roughly translates to "thought", and Muninn to "memory" or "mind" (although these are very rough translations).
