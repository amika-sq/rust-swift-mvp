//
// Copyright © Square, Inc. All rights reserved.
//

import SwiftUI

struct ContentView: View {
  @State private var value: Int32?

  var body: some View {
    VStack {
      Text("Rust Swift MVP").font(.title)
      Text(value != nil ? String(value!) : "")

      Button("Test") {
        let keyStore = SwiftKeyStore()
        let keyManager = KeyManager(keyStore: keyStore)
        let did = createDid(
          keyManager: keyManager,
          didMethod: .key,
          keyAlgorithm: .secp256k1
      )
        print("did uri: \(did.uri)")
      }
    }
    .padding()
  }
}

#Preview {
  ContentView()
}

class SwiftKeyStore: KeyStore {
  var mapping = [String: Jwk]();

  func get(key: String) throws -> Jwk? {
    return mapping[key]
  }

  func set(key: String, value: Jwk) throws {
    mapping[key] = value
  }
}
