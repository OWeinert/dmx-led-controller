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

    public async Task<StatusResponse> PushAnimationsAsync(IEnumerable<Animation> animations, CancellationToken cancellationToken = default)
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

    public async Task<StatusResponse> DrawOnLayerAsync(IEnumerable<Pixel> pixels, uint layerId, CancellationToken cancellationToken = default)
    {
        try
        {
            return await _client.DrawOnLayerAsync(pixels, layerId, cancellationToken);
        }
        catch (Exception ex)
        {
            _logger.LogError("{}", ex.Message);
            throw;
        }
    }

    public async Task<StatusResponse> DrawFullLayerAsync(SingleFrame frame, uint layerId, CancellationToken cancellationToken = default)
    {
        try
        {
            return await _client.DrawFullLayerAsync(frame, layerId, cancellationToken);
        }
        catch (Exception ex)
        {
            _logger.LogError("{}", ex.Message);
            throw;
        }
    }

    public async Task<StatusResponse> DrawDirectAsync(IEnumerable<Pixel> pixels, CancellationToken cancellationToken = default)
    {
        try
        {
            return await _client.DrawDirectAsync(pixels, cancellationToken);
        }
        catch (Exception ex)
        {
            _logger.LogError("{}", ex.Message);
            throw;
        }
    }

    public async Task<List<Animation>> GetAnimationQueueAsync(CancellationToken cancellationToken = default)
    {
        try
        {
            return await _client.GetAnimationQueueAsync(cancellationToken);
        }
        catch (Exception ex)
        {
            _logger.LogError("{}", ex.Message);
            throw;
        }
    }
}