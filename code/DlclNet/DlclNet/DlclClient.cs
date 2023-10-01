using DlclRpc;
using DlclNet.Models;
using Grpc.Net.Client;

namespace DlclNet;
public class DlclClient
{
    private readonly DlclDraw.DlclDrawClient _dlclDrawClient;
    
    public DlclClient(string address)
    {
        using var channel = GrpcChannel.ForAddress(address);
        _dlclDrawClient = new DlclDraw.DlclDrawClient(channel);
    }

    /// <summary>
    /// Retrieves an Array of Ids of the DLCL-Server's animated layers
    /// </summary>
    /// <param name="cancellationToken"></param>
    /// <returns></returns>
    public async Task<uint[]> GetAnimatedLayersAsync(CancellationToken cancellationToken = default)
    {
        var response = await _dlclDrawClient.GetAnimatedLayersAsync(new EmptyRequest(), cancellationToken: cancellationToken);
        var ids = response.Layers.ToArray();
        return ids;
    }

    /// <summary>
    /// Sends the list of <paramref name ="animations"/> as a stream to the DLCL-Server
    /// </summary>
    /// <param name="animations"></param>
    /// <param name="cancellationToken"></param>
    /// <returns></returns>
    public async Task<StatusResponse> PushAnimationsAsync(List<Animation> animations, CancellationToken cancellationToken = default)
    {
        var dtos = animations.ConvertAll(anim => anim.ToDTO());
        using var call = _dlclDrawClient.PushAnimations(cancellationToken: cancellationToken);
        foreach (var dto in dtos)
        {
            await call.RequestStream.WriteAsync(dto, cancellationToken);
        }
        var response = await call;
        return response;
    }
}
