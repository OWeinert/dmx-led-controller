using DlclNet.Models;
using DlclRpc;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.Logging;

namespace DlclNet.Hosting;

public sealed class DlclClientService : IDlclClientService
{
    public const string DlclConfigSection = "Dlcl";
    public const string DlclConfigAddress = "Address";

    private readonly DlclClient _client;
    
    private readonly ILogger<DlclClientService> _logger;
    
    public DlclClientService(IConfiguration config, ILogger<DlclClientService> logger)
    {
        _logger = logger;

        var key = string.Empty;
        try
        { 
            key = DlclConfigSection;
            var section = config.GetRequiredSection(key);

            key = DlclConfigAddress;
            var address = section.GetRequiredSection(key).Value;

            if (string.IsNullOrWhiteSpace(address))
            {
                const string msg = "Address configuration value is empty!";
                _logger.LogError("{}", msg);
                throw new ArgumentException(msg);
            }
            _client = new DlclClient(address!);
        }
        catch (Exception ex)
        {
            _logger.LogError("Configuration error in Section \"{key}\"!\n" +
                             "{ex}", key, ex.Message);
            throw;
        }
    }
    
    public async Task<uint[]> GetAnimatedLayersAsync(CancellationToken cancellationToken = default)
    {
        try
        {
            return await _client.GetAnimatedLayersAsync(cancellationToken);
        }
        catch (Exception ex)
        {
            _logger.LogError("{}", ex.Message);
            throw;
        }
    }

    public async Task<StatusResponse> PushAnimationsAsync(List<Animation> animations, CancellationToken cancellationToken = default)
    {
        try
        {
            return await _client.PushAnimationsAsync(animations, cancellationToken);
        }
        catch (Exception ex)
        {
            _logger.LogError("{}", ex.Message);
            throw;
        }
    }
}