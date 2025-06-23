chrome.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
  const bankSites = ["chase.com", "bankofamerica.com", "wellsfargo.com"];
  if (changeInfo.status === "complete" && tab.url) {
    const match = bankSites.some(site => tab.url.includes(site));
    if (match) {
      chrome.runtime.sendNativeMessage(
        "com.safevault.auth",
        { action: "trigger_auth" },
        (response) => {
          if (chrome.runtime.lastError) {
            console.error("SafeVault error:", chrome.runtime.lastError.message);
            console.log("❌ SafeVault authentication failed.");

            return;
          }

          if (response && response.status === "OK") {
            console.log("✅ SafeVault authentication passed.");
          } else {
            console.log("❌ SafeVault authentication failed.");
          }
        }
      );
    }
  }
});
